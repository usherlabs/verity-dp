use verity_client::client::{VerityClient, VerityClientConfig};

#[tokio::main()]
async fn main() -> Result<(), reqwest::Error> {
    println!("Proving a POST request using VerityClient...");

    let config = VerityClientConfig {
        prover_url: String::from("http://127.0.0.1:8080"),
        prover_zmq: String::from("tcp://127.0.0.1:5556"),
    };

    let verity_client = VerityClient::new(config);

    let result = verity_client
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
        .await;

    let response = match result {
        Ok(response) => response,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let json: serde_json::Value = response.subject.json().await.unwrap();
    println!("json: {:#?}", json);
    println!("response.proof.len(): {:#?}", response.proof.len());

    let result = verity_client
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
        .await;

    let response = match result {
        Ok(response) => response,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let json: serde_json::Value = response.subject.json().await.unwrap();
    println!("json: {:#?}", json);
    println!("response.proof.len(): {:#?}", response.proof.len());

    Ok(())
}
