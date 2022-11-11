//! Derived from Cranelift/wasm code.
//!
//! Translation skeleton that traverses the whole WebAssembly module and call helper functions
//! to deal with each part of it.

use c2zk_frontend_shared::FuncBuilder;
use c2zk_ir::ir;

use crate::{
    code_translator::translate_operator, error::WasmResult, module_trans_env::ModuleTranslationEnv,
};
use wasmparser::{
    FuncValidator, FunctionBody, Parser, Payload, Type, Validator, ValidatorResources,
};

/// Translate a sequence of bytes forming a valid Wasm binary into a list of valid IR
pub fn translate_module(data: &[u8]) -> WasmResult<ir::Module> {
    let mut module = ir::Module::new();
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
                dbg!(
                    "Function section: {:?}",
                    functions.into_iter().collect::<Vec<_>>()
                );
                // todo!()
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
                // todo!()
            }

            Payload::CodeSectionEntry(body) => {
                let mut func_validator = validator
                    .code_section_entry(&body)?
                    .into_validator(Default::default());
                parse_code_section_entry(
                    &mut module,
                    &mut module_translation_env,
                    &mut func_validator,
                    body,
                )?;
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

fn parse_code_section_entry(
    module: &mut ir::Module,
    module_translation_env: &mut ModuleTranslationEnv,
    validator: &mut FuncValidator<ValidatorResources>,
    body: FunctionBody,
) -> WasmResult<()> {
    let mut builder = FuncBuilder::new();
    let mut reader = body.get_binary_reader();
    // take care of wasm parameters
    // take care of wasm func locals
    //
    while !reader.eof() {
        let pos = reader.original_position();
        let op = reader.read_operator()?;
        validator.op(pos, &op)?;
        translate_operator(validator, &op, &mut builder)?;
    }
    module.push_func(builder.finish());
    Ok(())
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        let wat = r#"
            (module (func (param i32) (result i32)
              i32.const 1
              return))"#;
        let data = wat::parse_str(wat).unwrap();
        translate_module(data.as_ref()).unwrap();
    }
}
