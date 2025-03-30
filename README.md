# Example of core proof verification
## Technologies

- **Frontend**: HTML, CSS, JavaScript
- **Backend**: Node.js, Express
- **ZK Proof**: SP1 (Succinct Labs)
- **Programming Language**: Rust, JavaScript

## Local Setup for Real SP1 Proofs


### Installation Steps

1. Clone this repository:
   ```
   git clone https://github.com/Shershulia/example-of-core-proof
   cd script
   ```

2. Install SP1 toolchain (if not already installed):
   ```
   curl -L https://sp1up.succinct.xyz | bash
   sp1up
   ```

3. Run sp1 proof generation:
   ```
   cargo run --bin prove --release -- --prove --n 11

   ```

## License

MIT