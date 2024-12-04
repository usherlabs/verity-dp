use verity_client::client::{VerityClient, VerityClientConfig};

#[tokio::main()]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("Proving a GET request using VerityClient...");

    let config = VerityClientConfig {
        prover_url: String::from("http://127.0.0.1:8080"),
    };

    let response = VerityClient::new(config)
        .get("https://jsonplaceholder.typicode.com/posts/98")
        .redact(String::from("res:body:dolor"))
        .send()
        .await?;

    if response.subject.status().is_success() {
        let json: serde_json::Value = response.subject.json().await.unwrap();
        println!("json: {:#?}", json);
        println!("response.proof.len(): {:#?}", response.proof.len());
    } else {
        anyhow::bail!(response.subject.status());
    }

    Ok(())
}
