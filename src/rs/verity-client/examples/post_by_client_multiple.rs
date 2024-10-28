use verity_client::client::{VerityClient, VerityClientConfig};

#[tokio::main()]
async fn main() -> Result<(), reqwest::Error> {
    println!("Proving a POST request using VerityClient...");

    let config = VerityClientConfig {
        prover_url: String::from("http://127.0.0.1:8080"),
        prover_zmq: String::from("tcp://127.0.0.1:5556"),
    };

    let verity_client = VerityClient::new(config);

    let response = verity_client
        .post(String::from("https://jsonplaceholder.typicode.com/posts"))
        .json(&serde_json::json!({
            "userId": 1000,
            "firstName": "John",
            "lastName": "Smith",
            "fullName": "John Smith",
            "favoriteActor": "Johnny Depp"
        }))
        .redact(String::from("req:body:firstName, res:body:firstName"))
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = response.subject.json().await.unwrap();
    println!("{:#?}", json);

    let response = verity_client
        .post(String::from("https://jsonplaceholder.typicode.com/posts"))
        .json(&serde_json::json!({
            "userId": 1000,
            "firstName": "John",
            "lastName": "Smith",
            "fullName": "John Smith",
            "favoriteActor": "Johnny Depp"
        }))
        .redact(String::from("req:body:firstName, res:body:firstName"))
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = response.subject.json().await.unwrap();
    println!("{:#?}", json);

    Ok(())
}
