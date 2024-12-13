# Verity Data Processor (VDP) - Internet Computer (IC) Libraries

![License](https://img.shields.io/crates/l/verity_ic) [![verity_ic on crates.io](https://img.shields.io/crates/v/verity_ic)](https://crates.io/crates/verity_ic) [![verity_ic on docs.rs](https://docs.rs/verity_ic/badge.svg)](https://docs.rs/verity_ic)

# Verity IC - Documentation

## Overview
`verity_ic` is a Rust library crate designed for the Verity project, with functionalities tailored to interact with the Internet Computer (IC) ecosystem. This crate includes modules for cryptographic operations, owner management, random number generation, remittance logic, verification, and whitelist management.

## Features
- **Cryptographic Operations**: Perform IC-specific cryptographic tasks.
- **Owner Management**: Handle authenticated operations for canister ownership.
- **Random Number Initialization**: Generate random numbers securely on the IC.
- **Remittance Logic**: Manage remittance canister operations.
- **Whitelist Management**: Provide CRUD operations for managing principal-based whitelists.

## Modules

### 1. `crypto`
This module includes cryptographic utilities tailored for the Internet Computer.

#### Features:
- IC-compatible cryptographic operations.
- Secure handling of sensitive data.

### 2. `owner`
Manages canister ownership and ensures secure, authenticated interactions with canister methods.

#### Features:
- Define and update the canister owner.
- Guarded methods to ensure operations are owner-authenticated.

### 3. `random`
Provides functionality for generating and initializing random numbers for use in Internet Computer applications.

#### Features:
- Secure and IC-compatible random number generation.

### 4. `remittance`
Implements the logic for managing remittance operations on the Internet Computer.

#### Features:
- Simplified handling of remittance-related functionalities.
- Efficient integration with remittance canisters.

### 5. `verify`
This module includes methods for validating and verifying specific operations on the Internet Computer.

#### Features:
- Validation utilities for operations requiring additional checks.

### 6. `whitelist`
Provides CRUD operations for managing a whitelist. A whitelist is represented as a `HashMap` where each principal is mapped to a boolean value. A principal is considered whitelisted if its value is `true`.

#### Features:
- Add, remove, and query principals in the whitelist.
- Maintain and update whitelist states efficiently.

## Getting Started
### Installation
To include `verity_ic` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
verity_ic = "0.2.0"
```

### Usage Example
Below is a basic example demonstrating the usage of some modules:

```rust
use verity_ic::{crypto, owner, random, remittance, whitelist};

fn main() {
    // Example: Generating a random number
    let random_number = random::generate();
    println!("Random number: {}", random_number);

    // Example: Adding a principal to the whitelist
    let mut wl = whitelist::Whitelist::new();
    wl.add("principal-id", true);
    println!("Is whitelisted: {}", wl.is_whitelisted("principal-id"));
}
```

## Platform Support
This crate is specifically designed to interact with the Internet Computer and relies on IC-specific paradigms.

## License
`verity_ic` is licensed under the [MIT License](LICENSE).



For more information about the Verity Data Processor (VDP), visit the [VDP repository](https://github.com/usherlabs/verity-dp).
