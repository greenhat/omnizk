use c2zk_ir::ir::FuncIndex;

pub fn func_index_to_label(func_index: FuncIndex, func_names: &[String]) -> String {
    func_names
        .get(usize::from(func_index))
        .unwrap_or(&format!("f{}", u32::from(func_index)))
        .to_string()
}
