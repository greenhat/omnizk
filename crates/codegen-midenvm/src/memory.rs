use c2zk_ir::ir::Ty;

/// Miden memory layout.
/// Addresses start from the max and decrease as new values are stored.
/// Accomodating the space in the end of the available memory.
pub struct MidenMemoryLayout {
    /// The address of the first public input. Public inputs are saved from the stack on program launch.
    pub pub_inputs_start_address: i32,
    /// The address of the first public output. Public outputs are put on the stack when program finishes.
    pub pub_outputs_start_address: i32,
    /// The address of the first global variable. Global variables are stored in memory according to their index.
    pub globals_start_address: i32,
}

impl Default for MidenMemoryLayout {
    fn default() -> Self {
        let max_public_inputs = 1024;
        let max_public_outputs = 1024;
        let inputs_offset = 0;
        let outputs_offset = max_public_inputs * Ty::I64.size();
        let globals_offset = outputs_offset + max_public_outputs * Ty::I64.size();
        Self {
            pub_inputs_start_address: i32::MAX,
            pub_outputs_start_address: i32::MAX - inputs_offset,
            globals_start_address: i32::MAX - globals_offset,
        }
    }
}
