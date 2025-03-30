//! SP1 proof program for the Focus app.

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{SolType, private::FixedBytes};
use example_proof_lib::{verify_program, PublicValuesStruct};

pub fn main() {
    // Read input data
    let n = sp1_zkvm::io::read::<u32>();
    
    // Verification process
    let is_valid = verify_program(n);
    
    // Calculate n_squared
    let n_squared = n * n;
    
    // Create public values
    let public_values = PublicValuesStruct {
        n: n,
        n_squared: n_squared,
    };
    
    // Debug outputs
    println!("Example program verification:");
    println!("N: {}", n);
    println!("N squared: {}", n_squared);
    println!("Verification Result: {}", if is_valid { "SUCCESS" } else { "FAILED" });
    
    // Encode results and provide as output
    let bytes = PublicValuesStruct::abi_encode(&public_values);
    sp1_zkvm::io::commit_slice(&bytes);
}