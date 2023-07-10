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

use apint::ApInt;
use derive_more::Display;
use derive_more::From;
use derive_more::Into;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::dialects::builtin::attributes::VecAttr;

/// Frame pointer offset
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct FramePointer(i32);

#[derive(Copy, Clone, Default, Display)]
pub struct Mersenne31(i32);

impl Mersenne31 {
    pub fn as_i32(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Default)]
pub struct Operands([Mersenne31; 5]);

impl Operands {
    pub fn new_i32(a: i32, b: i32, c: i32, d: i32, e: i32) -> Self {
        Self([
            Mersenne31(a),
            Mersenne31(b),
            Mersenne31(c),
            Mersenne31(d),
            Mersenne31(e),
        ])
    }

    pub fn a(&self) -> Mersenne31 {
        self.0[0]
    }

    pub fn b(&self) -> Mersenne31 {
        self.0[1]
    }

    pub fn c(&self) -> Mersenne31 {
        self.0[2]
    }

    pub fn d(&self) -> Mersenne31 {
        self.0[3]
    }

    pub fn e(&self) -> Mersenne31 {
        self.0[4]
    }
}

impl From<Operands> for valida_machine::Operands<i32> {
    fn from(value: Operands) -> Self {
        valida_machine::Operands([
            value.0[0].0,
            value.0[1].0,
            value.0[2].0,
            value.0[3].0,
            value.0[4].0,
        ])
    }
}

impl TryFrom<&VecAttr> for Operands {
    type Error = String;

    fn try_from(value: &VecAttr) -> Result<Self, Self::Error> {
        let a = ApInt::from(
            value
                .0
                .get(0)
                .ok_or_else(|| "a not found".to_string())?
                .downcast_ref::<IntegerAttr>()
                .ok_or_else(|| "a IntegerAttr expected".to_string())?
                .clone(),
        )
        .try_to_i32()
        .map_err(|_| "expected i32")?;

        let b = ApInt::from(
            value
                .0
                .get(1)
                .ok_or_else(|| "b not found".to_string())?
                .downcast_ref::<IntegerAttr>()
                .ok_or_else(|| "b IntegerAttr expected".to_string())?
                .clone(),
        )
        .try_to_i32()
        .map_err(|_| "expected i32")?;

        let c = ApInt::from(
            value
                .0
                .get(2)
                .ok_or_else(|| "c not found".to_string())?
                .downcast_ref::<IntegerAttr>()
                .ok_or_else(|| "c IntegerAttr expected".to_string())?
                .clone(),
        )
        .try_to_i32()
        .map_err(|_| "expected i32")?;

        let d = ApInt::from(
            value
                .0
                .get(3)
                .ok_or_else(|| "d not found".to_string())?
                .downcast_ref::<IntegerAttr>()
                .ok_or_else(|| "d IntegerAttr expected".to_string())?
                .clone(),
        )
        .try_to_i32()
        .map_err(|_| "expected i32")?;

        let e = ApInt::from(
            value
                .0
                .get(4)
                .ok_or_else(|| "e not found".to_string())?
                .downcast_ref::<IntegerAttr>()
                .ok_or_else(|| "e IntegerAttr expected".to_string())?
                .clone(),
        )
        .try_to_i32()
        .map_err(|_| "expected i32")?;
        Ok(Operands::new_i32(a, b, c, d, e))
    }
}
