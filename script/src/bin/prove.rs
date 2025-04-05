use alloy_sol_types::SolType;
use clap::Parser;
use example_proof_lib::PublicValuesStruct;
use sp1_sdk::{
    include_elf, 
    ProverClient, 
    SP1Stdin, 
    HashableKey, 
    SP1ProofWithPublicValues,
    EnvProver
};
use hex;
use std::fs;
use std::path::Path;

/// RISC-V ELF file for the Focus proof program.
pub const EXAMPLE_PROOF_ELF: &[u8] = include_elf!("example_proof_program");

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    verify: bool,

    #[clap(long, default_value = "0")]
    n: u32,

    #[clap(long, default_value = "example_program_proof.bin")]
    proof_path: String,
}

fn verify_proof(client: &EnvProver, proof_path: &str) {
    println!("Verifying existing proof from file: {}", proof_path);
    let (_, vk) = client.setup(EXAMPLE_PROOF_ELF);
    
    // Load proof from file
    let proof = SP1ProofWithPublicValues::load(proof_path).expect("Failed to load proof");
    
    // Get public values from proof
    let decoded = PublicValuesStruct::abi_decode(proof.public_values.as_slice(), true).unwrap();
    let PublicValuesStruct { n, n_squared } = decoded;

    // Verify proof
    client.verify(&proof, &vk).expect("proof verification failed");
    println!("Proof successfully verified!");
    
    println!("\n=== Verification Details ===");
    println!("\nInput Data:");
    println!("  N: {}", n);
    
    println!("\nOutput Data:");
    println!("  N: {}", n);
    println!("  N squared: {}", n_squared);
    
    println!("\nPublic Values (Raw):");
    println!("  {}", hex::encode(&proof.public_values));
}

fn main() {
    // Setup logger
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse command line arguments
    let args = Args::parse();

    if args.execute == args.prove && args.prove == args.verify {
        eprintln!("Error: You must specify exactly one of --execute, --prove, or --verify");
        std::process::exit(1);
    }

    // Setup prover client
    let client = ProverClient::from_env();

    // Prepare inputs
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    println!("Example Program Data: N = {}", args.n);

    if args.execute {
        // Run program without generating proof
        let (output, report) = client.execute(EXAMPLE_PROOF_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read output
        let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        let PublicValuesStruct { n, n_squared } = decoded;
        
        println!("Example Program Verification Result:");
        println!("N: {}", n);
        println!("N squared: {}", n_squared);

        // Log executed instruction count
        println!("Number of instructions executed: {}", report.total_instruction_count());
    } else if args.verify {
        verify_proof(&client, &args.proof_path);
        println!("Your proof is valid! Welcome to the sp1 club buddy!")
    } else {
        // Setup program for proof generation
        let (pk, vk) = client.setup(EXAMPLE_PROOF_ELF);

        // Generate proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("proof generation failed");

        println!("Proof successfully generated!");

        // Verify proof
        client.verify(&proof, &vk).expect("proof verification failed");
        println!("Proof successfully verified!");
        
        // Save proof to disk
        let proof_path = "example_program_proof.bin";
        proof.save(proof_path).expect("failed to save proof");
        println!("Proof saved to file: {}", proof_path);

        // Read and display proof data
        let (output, _) = client.execute(EXAMPLE_PROOF_ELF, &stdin).run().unwrap();
        let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        let PublicValuesStruct { n, n_squared } = decoded;
        
        println!("\n=== Proof Details ===");
        
        println!("\nProof:");
        println!("  Size: {} bytes", proof_path.len());
        println!("  Path: {}", proof_path);
        
        println!("\nInput Data:");
        println!("  N: {}", args.n);

        println!("\nOutput Data:");
        println!("  N: {}", n);
        println!("  N squared: {}", n_squared);

        println!("Program VKey: {}", vk.bytes32());
        
        println!("\nPublic Values (Raw):");
        println!("  {}", hex::encode(&output));
        
    }
}