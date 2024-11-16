// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{collections::HashMap, env};

use risc0_build::{embed_methods_with_options, DockerOptions, GuestOptions};
use risc0_build_ethereum::generate_solidity_files;

use dotenv;

/// Paramters concerning the solidity code to be generated 
pub struct SolidityBuildParams {
    /// Path of where to save the generated imageID.sol file after running the build function
    pub solidity_image_id_path: String,
    /// Path of where to save the ELF.sol file after running the build
    pub solidity_elf_path: String,
}

impl Default for SolidityBuildParams {
     fn default() -> Self {
        Self {
            solidity_image_id_path: String::from("../contracts/ImageID.sol"),
            solidity_elf_path: String::from("../tests/Elf.sol"),
        }
    }
}

/// Run this method as a build hook in the guest repo
/// It is used to build the necessary solidity files to identify the circuit.
/// Make sure to add as a build dependency
pub fn build(build_params: SolidityBuildParams) {
    dotenv::dotenv().ok();

    // Builds can be made deterministic, and thereby reproducible, by using Docker to build the
    // guest. Check the RISC0_USE_DOCKER variable and use Docker to build the guest if set.
    let use_docker = env::var("RISC0_USE_DOCKER").ok().map(|_| DockerOptions {
        root_dir: Some("../".into()),
    });

    // Generate Rust source files for the methods crate.
    let guests = embed_methods_with_options(HashMap::from([(
        "guests",
        GuestOptions {
            features: Vec::new(),
            use_docker,
        },
    )]));

    // Generate Solidity source files for use with Forge.
    let solidity_opts = risc0_build_ethereum::Options::default()
        .with_image_id_sol_path(build_params.solidity_image_id_path)
        .with_elf_sol_path(build_params.solidity_elf_path);

    generate_solidity_files(guests.as_slice(), &solidity_opts).unwrap();
}
