# [Verity Client](https://github.com/usherlabs/verity-dp)

![License](https://img.shields.io/crates/l/verity-client) [![verity-client on crates.io](https://img.shields.io/crates/v/verity-client)](https://crates.io/crates/verity-client) [![verity-client on docs.rs](https://docs.rs/verity-client/badge.svg)](https://docs.rs/verity-client)

## Overview

`verity-client` is a Rust library for making notarized HTTP requests with built-in privacy controls.

## Installation

Install the library using Cargo:

1. Run the following command in your project directory:

```
cargo add verity-client
```

2. Or add to your `Cargo.toml`:

```toml
verity-client = "0.2.0"
```

## Quick Start

### GET Request

```rust
use verity_client::client::{VerityClient, VerityClientConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = VerityClientConfig {
        prover_url: "http://127.0.0.1:8080".to_string(),
    };

    let response = VerityClient::new(config)
        .get("https://jsonplaceholder.typicode.com/posts/98")
        .redact("res:body:dolor")
        .send()
        .await?;

    if response.subject.status().is_success() {
        let json: serde_json::Value = response.subject.json().await?;
        println!("Response: {:#?}", json);
    }

    Ok(())
}
```

### POST Request

```rust
use verity_client::client::{VerityClient, VerityClientConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = VerityClientConfig {
        prover_url: "http://127.0.0.1:8080".to_string(),
    };

    let response = VerityClient::new(config)
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "userId": 1000,
            "firstName": "John",
            "lastName": "Smith"
        }))
        .redact("req:body:firstName, res:body:firstName")
        .send()
        .await?;

    if response.subject.status().is_success() {
        let json: serde_json::Value = response.subject.json().await?;
        println!("Response: {:#?}", json);
    }

    Ok(())
}
```