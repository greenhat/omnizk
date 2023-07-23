use apint::ApInt;
use apint::Int;
use apint::Width;
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

use crate::types::FieldElemType;

pub type FieldElem = winter_math::fields::f64::BaseElement;

#[derive(PartialEq, Eq, Clone)]
pub struct FieldElemAttr {
    ty: Ptr<TypeObj>,
    val: FieldElem,
}
impl_attr!(FieldElemAttr, "FieldElem", "miden");

#[derive(Debug, Error)]
pub enum FieldElemError {
    #[error("ApInt {0:?} is too large to fit in a field element")]
    TooLarge(ApInt),
}

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

    pub fn from_integer_attr(
        ctx: &mut Context,
        int_attr: IntegerAttr,
    ) -> Result<FieldElemAttr, FieldElemError> {
        let ty = FieldElemType::get(ctx);
        if int_attr.get_type() == IntegerType::get(ctx, 32, Signedness::Signed) {
            Ok(FieldElemAttr::create(ty, apint_to_oxfoi(int_attr.into())))
        } else {
            Err(FieldElemError::TooLarge(int_attr.into()))
        }
    }
}

pub fn apint_to_oxfoi(value: ApInt) -> FieldElem {
    use winter_math::StarkField;
    assert!(value.width() <= 64.into());
    let i = Int::from(value);
    #[allow(clippy::expect_used)]
    let raw = i.try_to_i64().expect("failed to get 64-bit integer");
    if raw < 0 {
        FieldElem::new(FieldElem::MODULUS - raw.unsigned_abs())
    } else {
        FieldElem::new(raw as u64)
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
