use verity_client::client::{ VerityClient, VerityClientConfig };

#[tokio::main()]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("Proving a POST request using VerityClient...");

    let config = VerityClientConfig {
        prover_url: String::from("http://127.0.0.1:8080"),
    };

    let verity_client = VerityClient::new(config);

    let response = verity_client
        .post(String::from("https://jsonplaceholder.typicode.com/posts"))
        .json(
            &serde_json::json!({
            "userId": 1000,
            "firstName": "John",
            "lastName": "Smith",
            "fullName": "John Smith",
            "favoriteActor": "Johnny Depp"
        })
        )
        .redact(String::from("req:body:firstName, res:body:firstName"))
        .send().await?;

    if response.subject.status().is_success() {
        let json: serde_json::Value = response.subject.json().await.unwrap();
        println!("json: {:#?}", json);
        println!("response.proof.len(): {:#?}", response.proof.len());
    } else {
        anyhow::bail!(response.subject.status());
    }

    let response = verity_client
        .post(String::from("https://jsonplaceholder.typicode.com/posts"))
        .json(
            &serde_json::json!({
            "userId": 1000,
            "firstName": "John",
            "lastName": "Smith",
            "fullName": "John Smith",
            "favoriteActor": "Johnny Depp"
        })
        )
        .redact(String::from("req:body:firstName, res:body:firstName"))
        .send().await?;

    if response.subject.status().is_success() {
        let json: serde_json::Value = response.subject.json().await.unwrap();
        println!("json: {:#?}", json);
        println!("response.proof.len(): {:#?}", response.proof.len());
    } else {
        anyhow::bail!(response.subject.status());
    }

    Ok(())
}
