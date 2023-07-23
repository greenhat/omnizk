use ozk_wasm_dialect::types::MemAddress;

/// Miden memory layout.
/// Addresses start from the max and decrease as new values are stored.
/// Accomodating the space in the end of the available memory.
pub struct MidenMemoryLayout {
    /// The address of the first public input. Public inputs are saved from the stack on program launch.
    pub pub_inputs_start_address: i32,
    /// The address of the first public output. Public outputs are put on the stack when program finishes.
    pub pub_outputs_start_address: i32,
    /// The address of the first global variable. Global variables are stored in memory according to their index.
    pub globals_start_address: MemAddress,
}

impl Default for MidenMemoryLayout {
    fn default() -> Self {
        let max_public_inputs: u32 = 1024;
        let max_public_outputs: u32 = 1024;
        let inputs_offset: u32 = 0;
        let i64_size: u32 = 8;
        let outputs_offset: u32 = max_public_inputs * i64_size;
        let globals_offset: u32 = outputs_offset + max_public_outputs * i64_size;
        Self {
            pub_inputs_start_address: i32::MAX,
            pub_outputs_start_address: i32::MAX - inputs_offset as i32,
            globals_start_address: ((i32::MAX - globals_offset as i32) as u32).into(),
        }
    }
}
