use risc0_zkvm::guest::env;
use verity_verifier::verify_proof;

fn main() {
    let proof: String = env::read();

    let (recv, sent) = verify_proof(&proof).unwrap();

    env::commit(&recv);
    env::commit(&sent);
}
