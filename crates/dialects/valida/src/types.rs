// use pliron::common_traits::DisplayWithContext;
// use pliron::common_traits::Verify;
// use pliron::context::Context;
// use pliron::context::Ptr;
// use pliron::error::CompilerError;
// use pliron::impl_type;
// use pliron::r#type::Type;
// use pliron::r#type::TypeObj;
// use pliron::storage_uniquer::TypeValueHash;

// #[derive(Hash, PartialEq, Eq, Copy, Clone)]
// pub enum Field {
//     /// a 64-bit prime field defined by modulus p = 2^64 - 2^32 + 1,
//     /// all values that the VM operates with are field elements in this field (
//     /// i.e., values between 0 and 2^64 −2^32 , both inclusive).
//     /// Used in Miden VM and Triton VM
//     Oxfoi,
// }

// /// Field element type
// #[derive(Hash, PartialEq, Eq)]
// pub struct FieldElemType {
//     // TODO: remove?
//     field: Field,
// }
// impl_type!(FieldElemType, "felt", "ozk");

// impl FieldElemType {
//     /// Get or create a new felt type.
//     pub fn get(ctx: &mut Context, field: Field) -> Ptr<TypeObj> {
//         Type::register_instance(FieldElemType { field }, ctx)
//     }
//     /// Get, if it already exists, an felt type.
//     pub fn get_existing(ctx: &Context, field: Field) -> Option<Ptr<TypeObj>> {
//         Type::get_instance(FieldElemType { field }, ctx)
//     }

//     /// Get width.
//     pub fn get_field(&self) -> Field {
//         self.field
//     }
// }

// impl DisplayWithContext for FieldElemType {
//     fn fmt(&self, _ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "felt")
//     }
// }

// impl Verify for FieldElemType {
//     fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
//         todo!()
//     }
// }

// pub(crate) fn register(dialect: &mut pliron::dialect::Dialect) {
//     FieldElemType::register_type_in_dialect(dialect);
// }
