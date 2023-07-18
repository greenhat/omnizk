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
use thiserror::Error;

/// Frame pointer offset
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct FramePointer(i32);

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct ProgramCounter(u32);

impl From<ProgramCounter> for i32 {
    fn from(value: ProgramCounter) -> Self {
        value.0 as i32
    }
}

#[derive(Copy, Clone, Default, Display, PartialEq, Eq)]
pub struct Mersenne31(i32);

impl Mersenne31 {
    pub const ZERO: Self = Self(0);

    pub fn as_i32(self) -> i32 {
        self.0
    }
}

#[derive(Clone, Error, Debug)]
pub enum Mersenne31Error {
    #[error("invalid value: {0}")]
    InvalidValue(String),
}

impl TryFrom<&IntegerAttr> for Mersenne31 {
    type Error = Mersenne31Error;

    fn try_from(attr: &IntegerAttr) -> Result<Self, Self::Error> {
        let value = ApInt::from(attr.clone())
            .try_to_i32()
            .map_err(|e| Mersenne31Error::InvalidValue(format!("failed to get i32: {:?}", e)))?;
        Ok(Mersenne31(value))
    }
}

#[derive(Copy, Clone, Default)]
pub struct Operands([Mersenne31; 5]);

impl Operands {
    pub fn new<T: Into<Mersenne31>>(a: T, b: T, c: T, d: T, e: T) -> Self {
        Self([a.into(), b.into(), c.into(), d.into(), e.into()])
    }

    pub fn from_i32(a: i32, b: i32, c: i32, d: i32, e: i32) -> Self {
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

    pub fn set_b(&mut self, value: i32) {
        self.0[1] = Mersenne31(value);
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
        Ok(Operands::from_i32(a, b, c, d, e))
    }
}
