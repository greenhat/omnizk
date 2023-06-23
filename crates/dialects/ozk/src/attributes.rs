use apint::ApInt;
use apint::Int;
use apint::Width;
use derive_more::Display;
use derive_more::From;
use pliron::attribute::Attribute;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::attr_interfaces::TypedAttrInterface;
use pliron::error::CompilerError;
use pliron::impl_attr;
use pliron::r#type::TypeObj;
use winter_math::fields::f64::BaseElement as OxfoiFieldElem;
use winter_math::StarkField;

use intertrait::cast_to;
use pliron::with_context::AttachContext;

pub type ValidaFieldElem = u32;

// TODO: split into two ElemAttr types?
#[derive(PartialEq, Eq, Clone, Display, Debug, From)]
pub enum FieldElem {
    Oxfoi(OxfoiFieldElem),
    P231m1(ValidaFieldElem),
}

/// An attribute containing a FieldElement.
/// Similar to MLIR's [IntegerAttr](https://mlir.llvm.org/docs/Dialects/Builtin/#integerattr).
#[derive(PartialEq, Eq, Clone)]
pub struct FieldElemAttr {
    ty: Ptr<TypeObj>,
    val: FieldElem,
}
impl_attr!(FieldElemAttr, "FieldElem", "ozk");

impl DisplayWithContext for FieldElemAttr {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.val, self.ty.with_ctx(ctx))
    }
}

impl Verify for FieldElemAttr {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

impl FieldElemAttr {
    /// Create a new [FieldElemAttr].
    pub fn create(ty: Ptr<TypeObj>, val: FieldElem) -> Self {
        FieldElemAttr { ty, val }
    }
}

impl From<FieldElemAttr> for FieldElem {
    fn from(value: FieldElemAttr) -> Self {
        value.val
    }
}

#[cast_to]
impl TypedAttrInterface for FieldElemAttr {
    fn get_type(&self) -> Ptr<TypeObj> {
        self.ty
    }
}

pub(crate) fn register(dialect: &mut pliron::dialect::Dialect) {
    FieldElemAttr::register_attr_in_dialect(dialect);
}

pub fn apint64_to_field_elem(value: ApInt) -> FieldElem {
    assert!(value.width() <= 64.into());
    let i = Int::from(value);
    #[allow(clippy::expect_used)]
    let raw = i.try_to_i64().expect("64-bit integer");
    felt_i64(raw)
}

pub fn felt_i64(v: i64) -> FieldElem {
    if v < 0 {
        OxfoiFieldElem::new(OxfoiFieldElem::MODULUS - v.unsigned_abs()).into()
    } else {
        OxfoiFieldElem::new(v as u64).into()
    }
}
