use verity_client::client::{VerityClient, VerityClientConfig};

#[tokio::main()]
async fn main() -> Result<(), reqwest::Error> {
    println!("Proving a GET request using VerityClient...");

    let config = VerityClientConfig {
        prover_url: String::from("http://127.0.0.1:8080"),
        prover_zmq: String::from("tcp://127.0.0.1:5556"),
    };

    let response = VerityClient::new(config)
        .get("https://jsonplaceholder.typicode.com/posts/98")
        .redact(String::from("res:body:dolor"))
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = response.subject.json().await.unwrap();
    println!("{:#?}", json);

    Ok(())
}
