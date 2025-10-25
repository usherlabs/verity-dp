use std::time::Duration;

use axum::response::sse::{Event as SseEvent, Sse};
use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures_util::stream;
use std::convert::Infallible;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use verity_client::client::{NotaryInformation, VerityClient, VerityClientConfig};

async fn spawn_mock_server() -> (String, JoinHandle<()>) {
    // Bind to a random local port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let app = Router::new()
        .route(
            "/notaryinfo",
            get(|| async {
                let info = NotaryInformation {
                    version: "test".to_string(),
                    public_key: "-----BEGIN PUBLIC KEY---...".to_string(),
                    git_commit_hash: "0000000000000000000000000000000000000000".to_string(),
                    git_commit_timestamp: "0".to_string(),
                };
                Json(info)
            }),
        )
        .route("/proxy", post(proxy_handler).get(proxy_handler))
        .route("/proof/:id", get(proof_handler));

    let handle = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    (format!("http://{}", addr), handle)
}

async fn proxy_handler() -> impl IntoResponse {
    // Return a response header to indicate proof will arrive on SSE
    let mut headers = HeaderMap::new();
    headers.insert("T-PROOF-ID", HeaderValue::from_static("1"));
    (StatusCode::OK, headers, "ok")
}

async fn proof_handler(Path(id): Path<String>) -> impl IntoResponse {
    // Simulate SSE stream. For simplicity, send a single event and close.
    // Format: notary_pub_key|proof
    let body = format!(
        "data: {}|proof-for-{}\n\n",
        "-----BEGIN PUBLIC KEY---...", id
    );
    let headers = {
        let mut h = HeaderMap::new();
        h.insert(
            "content-type",
            HeaderValue::from_static("text/event-stream"),
        );
        h
    };
    (headers, body)
}

#[tokio::test]
async fn get_notary_info_works() {
    let (base, _server) = spawn_mock_server().await;
    let client = VerityClient::new(VerityClientConfig {
        prover_url: base,
        proof_timeout: Some(Duration::from_millis(3000)),
    });

    let info = client.get_notary_info().await.unwrap();
    assert_eq!(info.git_commit_hash.len(), 40);
    assert!(info.public_key.starts_with("-----BEGIN PUBLIC KEY"));
}

#[tokio::test]
async fn get_and_post_request_with_proof() {
    let (base, _server) = spawn_mock_server().await;
    let client = VerityClient::new(VerityClientConfig {
        prover_url: base.clone(),
        proof_timeout: Some(Duration::from_millis(3000)),
    });

    // GET
    let res = client
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await
        .unwrap();
    assert_eq!(res.subject.status(), StatusCode::OK);
    assert!(res.notary_pub_key.starts_with("-----BEGIN PUBLIC KEY"));
    assert!(res.proof.starts_with("proof-for-"));

    // POST
    let res = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({"title":"foo","body":"bar","userId":1}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.subject.status(), StatusCode::OK);
    assert!(res.notary_pub_key.starts_with("-----BEGIN PUBLIC KEY"));
    assert!(res.proof.starts_with("proof-for-"));
}

#[tokio::test]
async fn no_proof_header_returns_immediately() {
    use axum::routing::get as axget;
    use axum::Router as AxRouter;
    use tokio::net::TcpListener as TokioListener;

    // Custom server where /proxy does NOT include T-PROOF-ID
    let listener = TokioListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    async fn proxy_no_proof() -> impl IntoResponse {
        (StatusCode::OK, "ok")
    }

    async fn proof_never() -> impl IntoResponse {
        // Never send SSE events; keep the connection open
        let pending = stream::pending::<Result<SseEvent, Infallible>>();
        Sse::new(pending)
    }

    let app = AxRouter::new()
        .route("/proxy", axget(proxy_no_proof).post(proxy_no_proof))
        .route("/proof/:id", axget(proof_never))
        .route(
            "/notaryinfo",
            axget(|| async {
                Json(NotaryInformation {
                    version: "test".to_string(),
                    public_key: "-----BEGIN PUBLIC KEY---...".to_string(),
                    git_commit_hash: "0000000000000000000000000000000000000000".to_string(),
                    git_commit_timestamp: "0".to_string(),
                })
            }),
        );

    let _handle = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap()
    });

    let client = VerityClient::new(VerityClientConfig {
        prover_url: format!("http://{}", addr),
        proof_timeout: Some(Duration::from_millis(500)),
    });

    let res = client.get("https://example.com/").send().await.unwrap();

    assert_eq!(res.subject.status(), StatusCode::OK);
    assert_eq!(res.notary_pub_key, "");
    assert_eq!(res.proof, "");
}

#[tokio::test]
async fn proof_timeout_errors() {
    // Server that sets T-PROOF-ID but never emits SSE
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    async fn proxy_with_proof_header(mut headers: HeaderMap) -> impl IntoResponse {
        headers.insert("T-PROOF-ID", HeaderValue::from_static("1"));
        (StatusCode::OK, headers, "ok")
    }

    async fn proof_never() -> impl IntoResponse {
        // Never send SSE events; keep the connection open so client times out
        let pending = stream::pending::<Result<SseEvent, Infallible>>();
        Sse::new(pending)
    }

    let app = Router::new()
        .route(
            "/proxy",
            get(proxy_with_proof_header).post(proxy_with_proof_header),
        )
        .route(
            "/notaryinfo",
            get(|| async {
                Json(NotaryInformation {
                    version: "test".to_string(),
                    public_key: "-----BEGIN PUBLIC KEY---...".to_string(),
                    git_commit_hash: "0000000000000000000000000000000000000000".to_string(),
                    git_commit_timestamp: "0".to_string(),
                })
            }),
        )
        .route("/proof/:id", get(proof_never));

    let _handle = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap()
    });

    let client = VerityClient::new(VerityClientConfig {
        prover_url: format!("http://{}", addr),
        proof_timeout: Some(Duration::from_millis(200)),
    });

    let res = client.get("https://example.com/").send().await;
    assert!(res.is_err());
}
