use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::error::CompilerError;
use pliron::impl_type;
use pliron::r#type::Type;
use pliron::r#type::TypeObj;
use pliron::storage_uniquer::TypeValueHash;

/// Field element type
#[derive(Hash, PartialEq, Eq)]
pub struct FieldElemType;
impl_type!(FieldElemType, "felt", "miden");

impl FieldElemType {
    /// Get or create a new felt type.
    pub fn get(ctx: &mut Context) -> Ptr<TypeObj> {
        Type::register_instance(FieldElemType {}, ctx)
    }
    /// Get, if it already exists, an felt type.
    pub fn get_existing(ctx: &Context) -> Option<Ptr<TypeObj>> {
        Type::get_instance(FieldElemType {}, ctx)
    }
}

impl DisplayWithContext for FieldElemType {
    fn fmt(&self, _ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "felt")
    }
}

impl Verify for FieldElemType {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

pub(crate) fn register(dialect: &mut pliron::dialect::Dialect) {
    FieldElemType::register_type_in_dialect(dialect);
}
