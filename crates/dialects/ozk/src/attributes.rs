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
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::dialects::builtin::types::IntegerType;
use pliron::dialects::builtin::types::Signedness;
use pliron::error::CompilerError;
use pliron::impl_attr;
use pliron::r#type::TypeObj;

use intertrait::cast_to;
use pliron::with_context::AttachContext;
use thiserror::Error;

use crate::types::Field;
use crate::types::FieldElemType;

pub type ValidaFieldElem = u32;

#[derive(PartialEq, Eq, Clone, Display, Debug, From)]
// pub struct FieldElem(u64);
pub enum FieldElem {
    Oxfoi(winter_math::fields::f64::BaseElement),
    P231m1(u32),
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

pub fn oxfoi_field_elem_from_int(
    ctx: &mut Context,
    int_attr: IntegerAttr,
) -> Result<FieldElemAttr, FieldElemError> {
    let field_elem_type = FieldElemType::get(ctx, Field::Oxfoi);
    if int_attr.get_type() == IntegerType::get(ctx, 32, Signedness::Signed) {
        Ok(FieldElemAttr::create(
            field_elem_type,
            apint_to_oxfoi(int_attr.into()),
        ))
    } else {
        Err(FieldElemError::TooLarge(int_attr.into()))
    }
}

#[derive(Debug, Error)]
pub enum FieldElemError {
    #[error("ApInt {0:?} is too large to fit in a field element")]
    TooLarge(ApInt),
}

pub fn apint_to_oxfoi(value: ApInt) -> FieldElem {
    use winter_math::fields::f64::BaseElement as OxfoiFieldElem;
    use winter_math::StarkField;

    assert!(value.width() <= 64.into());
    let i = Int::from(value);
    #[allow(clippy::expect_used)]
    let raw = i.try_to_i64().expect("64-bit integer");
    if raw < 0 {
        OxfoiFieldElem::new(OxfoiFieldElem::MODULUS - raw.unsigned_abs()).into()
    } else {
        OxfoiFieldElem::new(raw as u64).into()
    }
}
