//! Derived from Cranelift/wasm code.
//!
//! Translation skeleton that traverses the whole WebAssembly module and call helper functions
//! to deal with each part of it.

use c2zk_frontend_shared::{FuncBuilder, ModuleBuilder};
use c2zk_ir::ir::{self, FuncIndex};

use crate::code_translator::translate_operator;
use crate::error::WasmError;
use crate::types::IntoIr;
use wasmparser::{
    BinaryReader, ExternalKind, FuncValidator, FunctionBody, NameSectionReader, Naming, Parser,
    Payload, Type, TypeRef, Validator, ValidatorResources, WasmModuleResources,
};

/// Translate a sequence of bytes forming a valid Wasm binary into a list of valid IR
pub fn translate_module(data: &[u8]) -> Result<ir::Module, WasmError> {
    let mut validator = Validator::new();
    let mut mod_builder = ModuleBuilder::new();

    for payload in Parser::new(0).parse_all(data) {
        // dbg!(&mod_builder);
        match payload? {
            Payload::Version {
                num,
                encoding,
                range,
            } => {
                validator.version(num, encoding, &range)?;
            }
            Payload::End(offset) => {
                validator.end(offset)?;
            }

            Payload::TypeSection(types) => {
                validator.type_section(&types)?;
                parse_type_section(types, &mut mod_builder)?;
            }

            Payload::ImportSection(imports) => {
                validator.import_section(&imports)?;
                parse_imports_section(imports, &mut mod_builder)?;
            }

            Payload::FunctionSection(functions) => {
                validator.function_section(&functions)?;
                // dbg!(
                //     "Function section: {:?}",
                //     functions.clone().into_iter().collect::<Vec<_>>()
                // );
                parse_function_section(functions, &mut mod_builder)?;
            }

            Payload::TableSection(tables) => {
                validator.table_section(&tables)?;
                dbg!(
                    "Table section: {:?}",
                    tables.into_iter().collect::<Vec<_>>()
                );
            }

            Payload::MemorySection(memories) => {
                validator.memory_section(&memories)?;
                // todo!()
            }

            Payload::TagSection(tags) => {
                validator.tag_section(&tags)?;
                dbg!("Tag section: {:?}", tags.into_iter().collect::<Vec<_>>());
                todo!()
            }

            Payload::GlobalSection(globals) => {
                validator.global_section(&globals)?;
                // todo!()
            }

            Payload::ExportSection(exports) => {
                validator.export_section(&exports)?;
                parse_export_section(exports, &mut mod_builder)?;
            }

            Payload::StartSection { func, range } => {
                validator.start_section(func, &range)?;
                // dbg!("Start section: {:?}", func);
                mod_builder.set_start_func(func);
            }

            Payload::ElementSection(elements) => {
                validator.element_section(&elements)?;
                todo!()
            }

            Payload::CodeSectionStart { count, range, .. } => {
                validator.code_section_start(count, &range)?;
                // dbg!("Code section start: {:?}", count);
                // todo!()
            }

            Payload::CodeSectionEntry(body) => {
                let mut func_validator = validator
                    .code_section_entry(&body)?
                    .into_validator(Default::default());
                parse_code_section_entry(&mut mod_builder, &mut func_validator, body)?;
            }

            Payload::DataSection(data) => {
                validator.data_section(&data)?;
                dbg!("Data section: {:?}", data.into_iter().collect::<Vec<_>>());
                todo!()
            }

            Payload::DataCountSection { count, range } => {
                validator.data_count_section(count, &range)?;
                todo!()
            }

            Payload::CustomSection(s) if s.name() == "name" => {
                let subsections = NameSectionReader::new(s.data(), s.data_offset());
                let result = parse_name_section(subsections, &mut mod_builder);
                if let Err(e) = result {
                    log::warn!("failed to parse name section {:?}", e);
                }
            }

            Payload::CustomSection(custom_section) => {
                dbg!("Custom section: {:?}", custom_section);
            }
            other => {
                validator.payload(&other)?;
                dbg!("Other: {:?}", other);
            }
        }
    }
    Ok(mod_builder.build()?)
}

fn parse_export_section(
    exports: wasmparser::ExportSectionReader,
    mod_builder: &mut ModuleBuilder,
) -> Result<(), WasmError> {
    for export in exports {
        let export = export?;

        #[allow(clippy::single_match)]
        match export.kind {
            ExternalKind::Func => {
                // dbg!(&export);
                if export.name == "__main" {
                    mod_builder.set_start_func(export.index);
                }
            }
            _ => {
                // dbg!(&export);
            }
        }
    }
    Ok(())
}

fn parse_type_section(
    types: wasmparser::TypeSectionReader,
    mod_builder: &mut ModuleBuilder,
) -> Result<(), WasmError> {
    for entry in types {
        match entry? {
            Type::Func(wasm_func_ty) => {
                mod_builder.push_type(wasm_func_ty.into_ir());
            }
        }
    }
    Ok(())
}

fn parse_code_section_entry(
    mod_builder: &mut ModuleBuilder,
    validator: &mut FuncValidator<ValidatorResources>,
    body: FunctionBody,
) -> Result<(), WasmError> {
    let func_idx = mod_builder.next_func_idx();
    let func_name = mod_builder
        .get_func_name(func_idx)
        .unwrap_or(format!("f{}", u32::from(func_idx)));
    // dbg!(&func_name);
    let mut builder = FuncBuilder::new(func_name);
    let mut reader = body.get_binary_reader();
    // take care of wasm parameters and pass the next local as num_params
    let num_params = mod_builder.get_func_type(func_idx)?.params.len();
    // dbg!(&num_params);
    parse_local_decls(&mut reader, &mut builder, num_params, validator)?;
    while !reader.eof() {
        // dbg!(&builder);
        let pos = reader.original_position();
        let op = reader.read_operator()?;
        // dbg!(&op);
        validator.op(pos, &op)?;
        translate_operator(validator, &op, &mut builder, mod_builder)?;
    }
    mod_builder.push_func_builder(builder);
    Ok(())
}

/// Parse the local variable declarations that precede the function body.
fn parse_local_decls(
    reader: &mut BinaryReader,
    builder: &mut FuncBuilder,
    _num_params: usize,
    validator: &mut FuncValidator<impl WasmModuleResources>,
) -> Result<(), WasmError> {
    let local_count = reader.read_var_u32()?;
    for _ in 0..local_count {
        let pos = reader.original_position();
        let count = reader.read_var_u32()?;
        let ty = reader.read::<wasmparser::ValType>()?;
        validator.define_locals(pos, count, ty)?;
        builder.declare_local(count, ty.into_ir());
    }
    Ok(())
}

fn parse_imports_section(
    imports: wasmparser::ImportSectionReader,
    mod_builder: &mut ModuleBuilder,
) -> Result<(), WasmError> {
    for entry in imports {
        let import = entry?;
        match import.ty {
            TypeRef::Func(type_index) => {
                mod_builder.push_import_func(type_index, import.module, import.name)?;
            }
            TypeRef::Memory(_ty) => {
                todo!()
            }
            TypeRef::Tag(_e) => {
                todo!()
            }
            TypeRef::Global(_ty) => {
                todo!()
            }
            TypeRef::Table(_ty) => {
                todo!()
            }
        }
    }
    Ok(())
}

pub fn parse_name_section(
    names: NameSectionReader,
    mod_builder: &mut ModuleBuilder,
) -> Result<(), WasmError> {
    for subsection in names {
        match subsection? {
            wasmparser::Name::Function(names) => {
                for name in names {
                    let Naming { index, name } = name?;
                    // don't demangle the function name cause it might clash with other func names
                    // including predefined funcs, pseudo ops funcs, etc.
                    mod_builder.declare_func_name(FuncIndex::from(index), name.to_string());
                }
            }
            wasmparser::Name::Module { .. } => {
                // environ.declare_module_name(name);
            }
            wasmparser::Name::Local(_reader) => {
                // for f in reader {
                //     let f = f?;
                //     if f.index == u32::max_value() {
                //         continue;
                //     }
                //     for name in f.names {
                //         let Naming { index, name } = name?;
                //         environ.declare_local_name(FuncIndex::from_u32(f.index), index, name)
                //     }
                // }
            }
            wasmparser::Name::Label(_)
            | wasmparser::Name::Type(_)
            | wasmparser::Name::Table(_)
            | wasmparser::Name::Global(_)
            | wasmparser::Name::Memory(_)
            | wasmparser::Name::Element(_)
            | wasmparser::Name::Data(_)
            | wasmparser::Name::Unknown { .. } => {}
        }
    }
    Ok(())
}

fn parse_function_section(
    functions: wasmparser::FunctionSectionReader,
    mod_builder: &mut ModuleBuilder,
) -> Result<(), WasmError> {
    for (func_idx, type_idx) in functions.into_iter().enumerate() {
        mod_builder.push_func_type(func_idx as u32, type_idx?);
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {}
