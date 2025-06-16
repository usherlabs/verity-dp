// use verity_dp_zk_host::builder::{self, SolidityBuildParams};

fn main() {
    // Default build
    risc0_build::embed_methods();

    // Solidity / EVM build
    // builder::build_for_evm(
    //     "verity-dp-examples-zktls-guest",
    //     SolidityBuildParams {
    //         solidity_image_id_path: "../ImageID.sol".into(),
    //         solidity_elf_path: "../Elf.sol".into(),
    //     },
    // );
}
