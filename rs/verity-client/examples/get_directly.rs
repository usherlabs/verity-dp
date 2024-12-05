#[tokio::main()]
async fn main() -> anyhow::Result<()> {
    let json: serde_json::Value = reqwest::Client
        ::new()
        .get("https://jsonplaceholder.typicode.com/posts/98")
        .send().await
        .unwrap()
        .json().await
        .unwrap();

    println!("{:#?}", json);

    Ok(())
}
