//! Derived from Cranelift/wasm code.
//!
//! Translation skeleton that traverses the whole WebAssembly module and call helper functions
//! to deal with each part of it.

use c2zk_frontend_shared::{FuncBuilder, ModuleBuilder};
use c2zk_ir::ir::{self, FuncIndex};

use crate::code_translator::translate_operator;
use crate::error::{WasmError, WasmResult};
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
                //     functions.into_iter().collect::<Vec<_>>()
                // );
                // todo!()
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
                let result = NameSectionReader::new(s.data(), s.data_offset())
                    .map_err(|e| e.into())
                    .and_then(|s| parse_name_section(s, &mut mod_builder));
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
) -> WasmResult<()> {
    for export in exports {
        let export = export?;
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
) -> WasmResult<()> {
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
) -> WasmResult<()> {
    // TODO: get the real function name
    // TODO: demangle the function name
    let func_idx = mod_builder.next_func_idx();
    // TODO: the name section parsed later
    let func_name = mod_builder
        .get_func_name(func_idx)
        .unwrap_or(format!("f{}", u32::from(func_idx)));
    dbg!(&func_name);
    let mut builder = FuncBuilder::new(func_name);
    let mut reader = body.get_binary_reader();
    // take care of wasm parameters and pass the next local as num_params
    let num_params = 0;
    parse_local_decls(&mut reader, &mut builder, num_params, validator)?;
    while !reader.eof() {
        // dbg!(&builder);
        let pos = reader.original_position();
        let op = reader.read_operator()?;
        // dbg!(&op);
        validator.op(pos, &op)?;
        translate_operator(validator, &op, &mut builder, mod_builder)?;
    }
    mod_builder.push_func(builder.build());
    Ok(())
}

/// Parse the local variable declarations that precede the function body.
fn parse_local_decls(
    reader: &mut BinaryReader,
    builder: &mut FuncBuilder,
    num_params: usize,
    validator: &mut FuncValidator<impl WasmModuleResources>,
) -> WasmResult<()> {
    let local_count = reader.read_var_u32()?;
    for _ in 0..local_count {
        let pos = reader.original_position();
        let count = reader.read_var_u32()?;
        let ty = reader.read_val_type()?;
        validator.define_locals(pos, count, ty)?;
        // TODO: add locals to builder
    }
    Ok(())
}

fn parse_imports_section(
    imports: wasmparser::ImportSectionReader,
    mod_builder: &mut ModuleBuilder,
) -> WasmResult<()> {
    for entry in imports {
        let import = entry?;
        match import.ty {
            TypeRef::Func(type_index) => {
                mod_builder.push_import_func(type_index, import.module, import.name)?;
            }
            TypeRef::Memory(ty) => {
                todo!()
            }
            TypeRef::Tag(e) => {
                todo!()
            }
            TypeRef::Global(ty) => {
                todo!()
            }
            TypeRef::Table(ty) => {
                todo!()
            }
        }
    }
    Ok(())
}

pub fn parse_name_section<'data>(
    names: NameSectionReader<'data>,
    mod_builder: &mut ModuleBuilder,
) -> WasmResult<()> {
    for subsection in names {
        match subsection? {
            wasmparser::Name::Function(names) => {
                for name in names {
                    let Naming { index, name } = name?;
                    mod_builder.declare_func_name(FuncIndex::from(index), name.to_string());
                }
            }
            wasmparser::Name::Module { name, .. } => {
                // environ.declare_module_name(name);
            }
            wasmparser::Name::Local(reader) => {
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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {}
