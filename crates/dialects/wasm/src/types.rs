use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::types::IntegerType;
use pliron::dialects::builtin::types::Signedness;
use pliron::r#type::TypeObj;

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
