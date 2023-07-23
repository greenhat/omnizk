use derive_more::From;
use derive_more::Into;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::types::IntegerType;
use pliron::dialects::builtin::types::Signedness;
use pliron::error::CompilerError;
use pliron::impl_type;
use pliron::r#type::Type;
use pliron::r#type::TypeObj;
use pliron::storage_uniquer::TypeValueHash;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum Field {
    /// a 64-bit prime field defined by modulus p = 2^64 - 2^32 + 1,
    /// all values that the VM operates with are field elements in this field (
    /// i.e., values between 0 and 2^64 −2^32 , both inclusive).
    Oxfoi,
    /// a 32-bit prime field defined by modulus p = 2^31 - 1,
    P231m1,
}

/// Field element type
#[derive(Hash, PartialEq, Eq)]
pub struct FieldElemType {
    field: Field,
}
impl_type!(FieldElemType, "felt", "ozk");

impl FieldElemType {
    /// Get or create a new felt type.
    pub fn get(ctx: &mut Context, field: Field) -> Ptr<TypeObj> {
        Type::register_instance(FieldElemType { field }, ctx)
    }
    /// Get, if it already exists, an felt type.
    pub fn get_existing(ctx: &Context, field: Field) -> Option<Ptr<TypeObj>> {
        Type::get_instance(FieldElemType { field }, ctx)
    }

    /// Get width.
    pub fn get_field(&self) -> Field {
        self.field
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

pub fn i32_type(ctx: &mut Context) -> Ptr<TypeObj> {
    IntegerType::get(ctx, 32, Signedness::Signed)
}

pub fn u32_type(ctx: &mut Context) -> Ptr<TypeObj> {
    IntegerType::get(ctx, 32, Signedness::Unsigned)
}

pub fn u32_type_unwrapped(ctx: &Context) -> Ptr<TypeObj> {
    #[allow(clippy::unwrap_used)]
    IntegerType::get_existing(ctx, 32, Signedness::Unsigned).unwrap()
}

pub fn i64_type(ctx: &mut Context) -> Ptr<TypeObj> {
    IntegerType::get(ctx, 64, Signedness::Signed)
}

/// Symbol name type of a function
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into)]
pub struct FuncSym(String);

impl AsRef<str> for FuncSym {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for FuncSym {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
