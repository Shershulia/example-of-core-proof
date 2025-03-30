use alloy_sol_types::sol;

sol! {
    /// Structure containing program output that can be easily deserialized by Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 n_squared;
        
    }
}

/// Verifies the focus session
pub fn verify_program(n: u32) -> bool {
    n >= 10
}