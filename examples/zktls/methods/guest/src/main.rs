#![no_main]

use risc0_zkvm::guest::env;
use verity_verify_tls::{verify_private_facets, PresentationBatch};

risc0_zkvm::guest::entry!(main);

fn main() {
    let input_bytes_len: usize = env::read();
    let mut input_bytes = vec![0u8; input_bytes_len];

    env::read_slice(&mut input_bytes);

    let batches: Vec<PresentationBatch> = bincode::deserialize(&input_bytes).unwrap();

    let (proof, payload_batches) = verify_private_facets(batches).unwrap();

    // process payload batches and produce arbitrary output data
    let data = bincode::serialize(&payload_batches).unwrap();

    let output_bytes = &bincode::serialize(&(data, proof)).unwrap();
    env::commit(output_bytes);
}
