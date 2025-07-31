use verity_verify_remote::{
    config::Config,
    ic::{Verifier, DEFAULT_IC_GATEWAY_LOCAL},
};
use verity_verify_tls::PayloadBatch;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rv_config = Config::new(
        DEFAULT_IC_GATEWAY_LOCAL.to_string(),
        verity_fixtures::ic::IDENTITY_PATH.to_string(),
        verity_fixtures::ic::ZKTLS_VERIFIER.to_string(),
    );

    let remote_verifier = Verifier::from_config(&rv_config).await?;

    let reply = remote_verifier
        .verify_receipt(verity_fixtures::receipt::RECEIPT_1KB.to_vec())
        .await?;

    let batches = bincode::deserialize::<Vec<PayloadBatch>>(&reply.data)?;

    println!("batches: {:?}", batches);
    println!("signature: {:?}", hex::encode(reply.signature));

    Ok(())
}
