# FeldmanVSS - Verifiable Secret Sharing System

This project implements a Feldman Verifiable Secret Sharing (VSS) scheme in Rust. It allows secure sharing of a secret among multiple participants, where only a subset of those participants (meeting a minimum threshold) can reconstruct the secret. Feldman's VSS ensures that each share can be verified for correctness using cryptographic commitments.

## Features

- **Secret Sharing**: Splits a secret into multiple shares
- **Verifiable Shares**: Ensures each share is valid using commitments
- **Reconstruction**: Reconstructs the original secret using only a subset of shares (threshold-based)
- **Modular Structure**: Clean, maintainable, and reusable code

## How it Works

### Secret Splitting:
- A secret is split into N shares using a polynomial of degree threshold - 1
- Each share corresponds to an evaluation of the polynomial at a given point

### Commitments:
- Each polynomial coefficient has a corresponding commitment to ensure verifiability
- Parties can use these commitments to verify the validity of their share

### Verification:
- Before reconstruction, each share can be verified using the commitments generated during the split process

### Reconstruction:
- Any subset of at least threshold shares can be used to reconstruct the original secret using Lagrange interpolation

## Folder Structure

```
project_root/
│
├── src/
│   ├── main.rs             # Entry point, contains the main logic
│   ├── lib.rs              # Exposes the FeldmanVSS library
│   ├── feldman_vss/        # Contains all the core modules
│   │   ├── mod.rs          # Declares the FeldmanVSS struct and core methods
│   │   ├── shares.rs       # Logic for generating and verifying shares
│   │   ├── polynomial.rs   # Polynomial-related logic (evaluation, coefficients)
│   │   ├── utils.rs        # Utility functions (BigInt <-> String conversion)
│   └── tests/              # Integration tests for the entire library
│       └── feldman_vss_test.rs
├── Cargo.toml              # Rust package manifest
└── README.md              # This file (documentation)
```

## Usage

### 1. Prerequisites
Make sure you have Rust installed. If not, install it from Rust's official website.

### 2. Clone the Repository
```bash
git clone https://github.com/shanmukh0504/Feldman-Vss.git
cd feldman-vss
```

### 3. Build the Project
```bash
cargo build
```

### 4. Run the Application
```bash
cargo run
```

### 5. Run Tests
```bash
cargo test
```

## Code Overview

### 1. Main Logic (main.rs)
The main.rs file contains the primary workflow of the secret sharing process:
- Generate a prime number q and a generator g
- Split the secret into multiple shares
- Verify the shares to ensure they are valid
- Reconstruct the original secret using a subset of shares

Example output:
```yaml
Shares:
Party 1: Share = 12345
Party 2: Share = 67890

Verifying Shares:
Share 1 is valid: true
Share 2 is valid: true

Reconstructed Secret: Gowtham is the best
```

### 2. FeldmanVSS Struct
This struct in feldman_vss/mod.rs provides the primary methods:
- `new()`: Generates a random prime q and a generator g
- `split()`: Splits the given secret into shares
- `verify()`: Verifies if a share is valid using commitments
- `reconstruct()`: Reconstructs the original secret from a subset of shares

### 3. Polynomial Logic (polynomial.rs)
This module manages polynomial generation and evaluation:
- `generate_coefficients()`: Creates random polynomial coefficients
- `evaluate_polynomial()`: Evaluates the polynomial at a given point

### 4. Share Management (shares.rs)
This module handles the creation and verification of shares:
- `generate_shares()`: Generates shares and commitments from the secret
- `verify_share()`: Verifies a given share using commitments

### 5. Utility Functions (utils.rs)
Contains helper functions for converting between BigInt and String:
- `bigint_to_string()`: Converts a BigInt to a String
- `string_to_bigint()`: Converts a String back to a BigInt

## Testing

We have two types of tests:
- **Unit Tests**: Validate the core functions (like polynomial evaluation, share generation, and verification)
- **Integration Tests**: Ensure the entire system works together correctly (splitting, verifying, and reconstructing the secret)

### Running Tests
```bash
cargo test
```

Expected output:
```arduino
running 6 tests
test feldman_vss::shares::tests::test_generate_shares ... ok
test feldman_vss::shares::tests::test_verify_share ... ok
test feldman_vss::polynomial::tests::test_generate_coefficients ... ok
test feldman_vss::polynomial::tests::test_evaluate_polynomial ... ok
test feldman_vss_test::test_split_and_verify_shares ... ok
test feldman_vss_test::test_reconstruct_secret ... ok

test result: ok. 6 passed; 0 failed; 0 ignored
```

## Security Considerations

- **Prime Generation**: Uses a 256-bit prime, which ensures cryptographic security
- **Commitments**: Polynomial coefficients are committed using BigInt, allowing for verifiable shares
- **Threshold**: Only the specified number of shares (threshold) can reconstruct the original secret

## Potential Improvements

- **Increase Prime Size**: Use larger primes (e.g., 512 or 1024 bits) for stronger security
- **Encryption of Shares**: Add optional encryption to shares to enhance security
- **Error Handling**: Improve error handling with more descriptive messages
- **Custom Generators**: Allow users to specify custom generators or group parameters

## Contributing

Contributions are welcome! Please follow these steps:
1. Fork the repository
2. Create a new feature branch
3. Make your changes and write tests
4. Submit a pull request with a detailed description