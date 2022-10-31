//! Derived from Cranelift/wasm code.
//!
//! Translation skeleton that traverses the whole WebAssembly module and call helper functions
//! to deal with each part of it.

use c2zk_ir::ir;

use crate::error::WasmResult;
use wasmparser::{NameSectionReader, Parser, Payload, Validator};

/// Translate a sequence of bytes forming a valid Wasm binary into a list of valid IR
pub fn translate_module(data: &[u8]) -> WasmResult<ir::Module> {
    let mut module = ir::Module::new();
    let mut validator = Validator::new();

    for payload in Parser::new(0).parse_all(data) {
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
                // parse_type_section(types, &mut module_translation_state, module)?;
            }

            Payload::ImportSection(imports) => {
                validator.import_section(&imports)?;
                // parse_import_section(imports, module)?;
            }

            Payload::FunctionSection(functions) => {
                validator.function_section(&functions)?;
                parse_function_section(functions, module)?;
            }

            Payload::TableSection(tables) => {
                validator.table_section(&tables)?;
                parse_table_section(tables, module)?;
            }

            Payload::MemorySection(memories) => {
                validator.memory_section(&memories)?;
                parse_memory_section(memories, module)?;
            }

            Payload::TagSection(tags) => {
                validator.tag_section(&tags)?;
                parse_tag_section(tags, module)?;
            }

            Payload::GlobalSection(globals) => {
                validator.global_section(&globals)?;
                parse_global_section(globals, module)?;
            }

            Payload::ExportSection(exports) => {
                validator.export_section(&exports)?;
                parse_export_section(exports, module)?;
            }

            Payload::StartSection { func, range } => {
                validator.start_section(func, &range)?;
                parse_start_section(func, module)?;
            }

            Payload::ElementSection(elements) => {
                validator.element_section(&elements)?;
                parse_element_section(elements, module)?;
            }

            Payload::CodeSectionStart { count, range, .. } => {
                validator.code_section_start(count, &range)?;
                module.reserve_function_bodies(count, range.start as u64);
            }

            Payload::CodeSectionEntry(body) => {
                let func_validator = validator
                    .code_section_entry(&body)?
                    .into_validator(Default::default());
                module.define_function_body(func_validator, body)?;
            }

            Payload::DataSection(data) => {
                validator.data_section(&data)?;
                parse_data_section(data, module)?;
            }

            Payload::DataCountSection { count, range } => {
                validator.data_count_section(count, &range)?;
                module.reserve_passive_data(count)?;
            }

            Payload::CustomSection(s) if s.name() == "name" => {
                let result = NameSectionReader::new(s.data(), s.data_offset())
                    .map_err(|e| e.into())
                    .and_then(|s| parse_name_section(s, module));
                if let Err(e) = result {
                    log::warn!("failed to parse name section {:?}", e);
                }
            }

            Payload::CustomSection(s) => module.custom_section(s.name(), s.data())?,
            other => {
                validator.payload(&other)?;
                panic!("unimplemented section {:?}", other);
            }
        }
    }

    Ok(module)
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let wat = r#"
            (module (func (param i32) (result i32)
              i32.const 1
              return))"#;
        let data = wat::parse_str(wat).unwrap();
        translate_module(data.as_ref()).unwrap();
    }
}
