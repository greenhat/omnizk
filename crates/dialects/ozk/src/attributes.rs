use pliron::attribute::AttrObj;
use pliron::attribute::Attribute;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::attr_interfaces::TypedAttrInterface;
use pliron::error::CompilerError;
use pliron::impl_attr;
use pliron::r#type::TypeObj;

use intertrait::cast_to;
use pliron::with_context::AttachContext;

pub type FieldElem = winter_math::fields::f64::BaseElement;

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
    pub fn create(ty: Ptr<TypeObj>, val: FieldElem) -> AttrObj {
        Box::new(FieldElemAttr { ty, val })
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
