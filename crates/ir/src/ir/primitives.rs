// Derived from wasmparser code

/// Types as defined [here].
///
/// [here]: https://webassembly.github.io/spec/core/syntax/types.html#types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
    ExnRef,
    Func,
    EmptyBlockType,
}

/// Either a value type or a function type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TypeOrFuncType {
    /// A value type.
    ///
    /// When used as the type for a block, this type is the optional result
    /// type: `[] -> [t?]`.
    Type(Type),

    /// A function type (referenced as an index into the types section).
    FuncType(u32),
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryImmediate {
    /// Alignment, stored as `n` where the actual alignment is `2^n`
    pub align: u8,

    /// A fixed byte-offset that this memory immediate specifies.
    ///
    /// Note that the memory64 proposal can specify a full 64-bit byte offset
    /// while otherwise only 32-bit offsets are allowed. Once validated
    /// memory immediates for 32-bit memories are guaranteed to be at most
    /// `u32::MAX` whereas 64-bit memories can use the full 64-bits.
    pub offset: u64,

    /// The index of the memory this immediate points to.
    ///
    /// Note that this points within the module's own memory index space, and
    /// is always zero unless the multi-memory proposal of WebAssembly is
    /// enabled.
    pub memory: u32,
}

/// A br_table entries representation.
#[derive(Clone, Debug)]
pub struct BrTable {
    pub(crate) targets: Vec<u32>,
    pub(crate) cnt: u32,
    pub(crate) default: u32,
}

/// An IEEE binary32 immediate floating point value, represented as a u32
/// containing the bitpattern.
///
/// All bit patterns are allowed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ieee32(pub(crate) u32);

impl Ieee32 {
    pub fn bits(self) -> u32 {
        self.0
    }
}

/// An IEEE binary64 immediate floating point value, represented as a u64
/// containing the bitpattern.
///
/// All bit patterns are allowed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ieee64(pub(crate) u64);

impl Ieee64 {
    pub fn bits(self) -> u64 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct V128(pub(crate) [u8; 16]);

impl V128 {
    pub fn bytes(&self) -> &[u8; 16] {
        &self.0
    }

    pub fn i128(&self) -> i128 {
        i128::from_le_bytes(self.0)
    }
}

pub type SIMDLaneIndex = u8;
