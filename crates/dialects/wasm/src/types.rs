//! derived from Cranelift/wasm-types
//!
//! Internal dependency of Wasmtime and OmniZK that defines types for
//! WebAssembly.

use derive_more::{Display, From, Into};
use ozk_ozk_dialect::types::{i32_type, i64_type};
use pliron::{
    context::{Context, Ptr},
    dialects::builtin::types::FunctionType,
    r#type::TypeObj,
};
pub use wasmparser;
use wasmparser::{BlockType, FuncType, RefType, ValType};

/// Index type of a function (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct FuncIndex(u32);

/// Index type of a defined function inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct DefinedFuncIndex(u32);

/// Index type of a defined table inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct DefinedTableIndex(u32);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct DefinedMemoryIndex(u32);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct OwnedMemoryIndex(u32);

/// Index type of a defined global inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct DefinedGlobalIndex(u32);

/// Index type of a table (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct TableIndex(u32);

/// Index type of a global variable (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct GlobalIndex(u32);

/// Index type of a linear memory (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct MemoryIndex(u32);

/// Index type of a signature (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct SignatureIndex(u32);

/// Index type of a passive data segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct DataIndex(u32);

/// Index type of a passive element segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct ElemIndex(u32);

/// Index type of a type inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct TypeIndex(u32);

/// Index type of an event inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct TagIndex(u32);

/// Address in the linear memory
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct MemAddress(u32);

/// Relative depth in Br* ops
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From, Into, Display)]
pub struct RelativeDepth(u32);

/// An index of an entity.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum EntityIndex {
    /// Function index.
    Function(FuncIndex),
    /// Table index.
    Table(TableIndex),
    /// Memory index.
    Memory(MemoryIndex),
    /// Global index.
    Global(GlobalIndex),
}

impl From<FuncIndex> for EntityIndex {
    fn from(idx: FuncIndex) -> EntityIndex {
        EntityIndex::Function(idx)
    }
}

impl From<TableIndex> for EntityIndex {
    fn from(idx: TableIndex) -> EntityIndex {
        EntityIndex::Table(idx)
    }
}

impl From<MemoryIndex> for EntityIndex {
    fn from(idx: MemoryIndex) -> EntityIndex {
        EntityIndex::Memory(idx)
    }
}

impl From<GlobalIndex> for EntityIndex {
    fn from(idx: GlobalIndex) -> EntityIndex {
        EntityIndex::Global(idx)
    }
}

impl From<usize> for FuncIndex {
    fn from(idx: usize) -> FuncIndex {
        FuncIndex(idx as u32)
    }
}

impl From<FuncIndex> for usize {
    fn from(value: FuncIndex) -> Self {
        value.0 as usize
    }
}

/// A type of an item in a wasm module where an item is typically something that
/// can be exported.
#[derive(Clone, Debug)]
pub enum EntityType {
    /// A global variable with the specified content type
    Global(Global),
    /// A linear memory with the specified limits
    Memory(Memory),
    /// An event definition.
    Tag(Tag),
    /// A table with the specified element type and limits
    Table(Table),
    /// A function type where the index points to the type section and records a
    /// function signature.
    Function(SignatureIndex),
}

/// A WebAssembly global.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Global {
    /// The Wasm type of the value stored in the global.
    pub wasm_ty: ValType,
    /// A flag indicating whether the value may change at runtime.
    pub mutability: bool,
    /// The source of the initial value.
    pub initializer: GlobalInit,
}

/// Globals are initialized via the `const` operators or by referring to another import.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum GlobalInit {
    /// An `i32.const`.
    I32Const(i32),
    /// An `i64.const`.
    I64Const(i64),
    /// An `f32.const`.
    F32Const(u32),
    /// An `f64.const`.
    F64Const(u64),
    /// A `vconst`.
    V128Const(u128),
    /// A `global.get` of another global.
    GetGlobal(GlobalIndex),
    /// A `ref.null`.
    RefNullConst,
    /// A `ref.func <index>`.
    RefFunc(FuncIndex),
    ///< The global is imported from, and thus initialized by, a different module.
    Import,
}

impl Global {
    /// Creates a new `Global` type from wasmparser's representation.
    #[allow(dead_code)]
    pub fn new(ty: wasmparser::GlobalType, initializer: GlobalInit) -> Global {
        Global {
            wasm_ty: ty.content_type,
            mutability: ty.mutable,
            initializer,
        }
    }
}

/// WebAssembly table.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Table {
    /// The table elements' Wasm type.
    pub wasm_ty: RefType,
    /// The minimum number of elements in the table.
    pub minimum: u32,
    /// The maximum number of elements in the table.
    pub maximum: Option<u32>,
}

impl From<wasmparser::TableType> for Table {
    fn from(ty: wasmparser::TableType) -> Table {
        Table {
            wasm_ty: ty.element_type,
            minimum: ty.initial,
            maximum: ty.maximum,
        }
    }
}

/// WebAssembly linear memory.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Memory {
    /// The minimum number of pages in the memory.
    pub minimum: u64,
    /// The maximum number of pages in the memory.
    pub maximum: Option<u64>,
    /// Whether the memory may be shared between multiple threads.
    pub shared: bool,
    /// Whether or not this is a 64-bit memory
    pub memory64: bool,
}

impl From<wasmparser::MemoryType> for Memory {
    fn from(ty: wasmparser::MemoryType) -> Memory {
        Memory {
            minimum: ty.initial,
            maximum: ty.maximum,
            shared: ty.shared,
            memory64: ty.memory64,
        }
    }
}

/// WebAssembly event.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Tag {
    /// The event signature type.
    pub ty: TypeIndex,
}

impl From<wasmparser::TagType> for Tag {
    fn from(ty: wasmparser::TagType) -> Tag {
        match ty.kind {
            wasmparser::TagKind::Exception => Tag {
                ty: TypeIndex(ty.func_type_idx),
            },
        }
    }
}

/// Convert a `wasmparser` type to a pliron type.
pub fn from_block_type(ctx: &mut Context, block_type: &BlockType) -> Ptr<TypeObj> {
    match block_type {
        BlockType::Empty => FunctionType::get(ctx, Vec::new(), Vec::new()),
        BlockType::Type(ty) => {
            let res_ty = from_val_type(ctx, ty);
            FunctionType::get(ctx, Vec::new(), vec![res_ty])
        }
        BlockType::FuncType(_) => todo!(),
    }
}

/// Convert a `wasmparser` type to a pliron type.
#[allow(clippy::unimplemented)]
pub fn from_val_type(ctx: &mut Context, val_type: &ValType) -> Ptr<TypeObj> {
    match val_type {
        ValType::I32 => i32_type(ctx),
        ValType::I64 => i64_type(ctx),
        ValType::F32 => unimplemented!("no support for floating types"),
        ValType::F64 => unimplemented!("no support for floating types"),
        ValType::V128 => todo!(),
        ValType::Ref(_) => todo!(),
    }
}

///  Convert a `wasmparser` type to a pliron type.
pub fn from_func_type(ctx: &mut Context, func_type: &FuncType) -> Ptr<TypeObj> {
    let params = func_type
        .params()
        .iter()
        .map(|ty| from_val_type(ctx, ty))
        .collect();
    let results = func_type
        .results()
        .iter()
        .map(|ty| from_val_type(ctx, ty))
        .collect();
    FunctionType::get(ctx, params, results)
}
