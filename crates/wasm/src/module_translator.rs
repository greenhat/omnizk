//! Derived from Cranelift/wasm code.
//!
//! Translation skeleton that traverses the whole WebAssembly module and call helper functions
//! to deal with each part of it.

use c2zk_ir::ir;

use crate::{error::WasmResult, module_trans_env::ModuleTranslationEnv};
use wasmparser::{Parser, Payload, Type, Validator};

/// Translate a sequence of bytes forming a valid Wasm binary into a list of valid IR
pub fn translate_module(data: &[u8]) -> WasmResult<ir::Module> {
    let module = ir::Module::new();
    let mut validator = Validator::new();
    let mut module_translation_env = ModuleTranslationEnv::new();

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
                parse_type_section(types, &mut module_translation_env)?;
            }

            Payload::ImportSection(imports) => {
                validator.import_section(&imports)?;
                todo!()
            }

            Payload::FunctionSection(functions) => {
                validator.function_section(&functions)?;
                todo!()
            }

            Payload::TableSection(tables) => {
                validator.table_section(&tables)?;
                todo!()
            }

            Payload::MemorySection(memories) => {
                validator.memory_section(&memories)?;
                todo!()
            }

            Payload::TagSection(tags) => {
                validator.tag_section(&tags)?;
                todo!()
            }

            Payload::GlobalSection(globals) => {
                validator.global_section(&globals)?;
                todo!()
            }

            Payload::ExportSection(exports) => {
                validator.export_section(&exports)?;
                todo!()
            }

            Payload::StartSection { func, range } => {
                validator.start_section(func, &range)?;
                todo!()
            }

            Payload::ElementSection(elements) => {
                validator.element_section(&elements)?;
                todo!()
            }

            Payload::CodeSectionStart { count, range, .. } => {
                validator.code_section_start(count, &range)?;
                todo!()
            }

            Payload::CodeSectionEntry(body) => {
                let _func_validator = validator
                    .code_section_entry(&body)?
                    .into_validator(Default::default());
                todo!()
            }

            Payload::DataSection(data) => {
                validator.data_section(&data)?;
                todo!()
            }

            Payload::DataCountSection { count, range } => {
                validator.data_count_section(count, &range)?;
                todo!()
            }

            Payload::CustomSection(s) if s.name() == "name" => {
                todo!()
            }

            Payload::CustomSection(_) => todo!(),
            other => {
                validator.payload(&other)?;
                todo!()
            }
        }
    }

    Ok(module)
}

fn parse_type_section(
    types: wasmparser::TypeSectionReader,
    module_translation_env: &mut ModuleTranslationEnv,
) -> WasmResult<()> {
    for entry in types {
        match entry? {
            Type::Func(wasm_func_ty) => {
                module_translation_env.types.push(wasm_func_ty);
            }
        }
    }
    Ok(())
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
