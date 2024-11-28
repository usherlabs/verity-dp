use std::str::FromStr;
use std::time::Duration;

use anyhow::anyhow;
use futures::stream::StreamExt;
use http::{HeaderValue, Method};
use reqwest::{IntoUrl, Response, Url};
use reqwest_eventsource::{Event, EventSource};
use tokio::select;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::request::RequestBuilder;
use crate::Error;

/// Time to wait for a proof received over ZMQ socket since receiving HTTP response
const PROOF_TIMEOUT: Duration = Duration::from_millis(5000);

#[derive(Clone)]
pub struct VerityClientConfig {
    pub prover_url: String,
}

#[derive(Clone)]
pub struct VerityClient {
    pub(crate) inner: reqwest::Client,
    pub(crate) config: VerityClientConfig,
}

pub struct VerityResponse {
    pub subject: Response,
    pub proof: String,
    pub notary_pub_key: String,
}

impl VerityClient {
    pub fn new(config: VerityClientConfig) -> Self {
        return Self {
            inner: reqwest::Client::new(),
            config,
        };
    }

    /// Convenience method to make a `GET` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    /// Convenience method to make a `POST` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    /// Start building a `Request` with the `Method` and `Url`.
    ///
    /// Returns a `RequestBuilder`, which will allow setting headers and
    /// the request body before sending.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        RequestBuilder {
            client: self.clone(),
            inner: self.inner.request(method, url),
        }
    }

    /// Executes a `Request`.
    ///
    /// A `Request` can be built manually with `Request::new()` or obtained
    /// from a RequestBuilder with `RequestBuilder::build()`.
    ///
    /// You should prefer to use the `RequestBuilder` and
    /// `RequestBuilder::send()`.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending request,
    /// redirect loop was detected or redirect limit was exhausted.
    pub async fn execute(
        &mut self,
        request: reqwest::Request,
    ) -> Result<VerityResponse, crate::Error> {
        self.execute_request(request).await
    }

    pub async fn execute_request(
        &mut self,
        mut req: reqwest::Request,
    ) -> Result<VerityResponse, crate::Error> {
        let proxy_url = &String::from(req.url().as_str());
        let headers = req.headers_mut();

        let request_id = Uuid::new_v4();
        headers.append(
            "T-REQUEST-ID",
            HeaderValue::from_str(&format!("{}", request_id)).unwrap(),
        );

        headers.append("T-PROXY-URL", HeaderValue::from_str(proxy_url).unwrap());

        *req.url_mut() = Url::from_str(&format!("{}/proxy", self.config.prover_url)).unwrap();

        let req = reqwest::RequestBuilder::from_parts(self.inner.clone(), req);

        let request_cancellation_token = CancellationToken::new();
        let timeout_cancellation_token = CancellationToken::new();

        let proof_awaiter = self.await_proof(
            request_id.to_string(),
            request_cancellation_token.clone(),
            timeout_cancellation_token.clone(),
        )?;

        let (response, proof_msg) = tokio::try_join!(
            self.send_request(req, request_cancellation_token, timeout_cancellation_token),
            proof_awaiter
        )
        .map_err(|e| anyhow!("Failed to prove the request: {}", e))?;

        let subject = response?;
        let (notary_pub_key, proof) = proof_msg?;

        Ok(VerityResponse {
            subject,
            proof,
            notary_pub_key,
        })
    }

    fn send_request(
        &self,
        request: reqwest::RequestBuilder,
        request_cancellation_token: CancellationToken,
        timeout_cancellation_token: CancellationToken,
    ) -> JoinHandle<Result<reqwest::Response, reqwest::Error>> {
        tokio::spawn(async move {
            let result = request.send().await?;

            if !result.status().is_success() {
                request_cancellation_token.cancel();
            } else {
                tokio::spawn(async move {
                    tokio::time::sleep(PROOF_TIMEOUT).await;
                    timeout_cancellation_token.cancel();
                });
            }

            Ok(result)
        })
    }

    fn await_proof(
        &self,
        request_id: String,
        request_cancellation_token: CancellationToken,
        timeout_cancellation_token: CancellationToken,
    ) -> Result<JoinHandle<Result<(String, String), Error>>, Error> {
        let url =
            Url::from_str(&format!("{}/proof/{}", self.config.prover_url, request_id)).unwrap();
        let mut event_source = EventSource::get(url);

        let awaiter = tokio::task::spawn(async move {
            while let Some(event) = event_source.next().await {
                println!("waiting for event...");
                match event {
                    Ok(Event::Open) => {}
                    Ok(Event::Message(message)) => {
                        let parts: Vec<&str> = message.data.splitn(3, "|").collect();

                        return Ok((parts[0].to_string(), parts[1].to_string()));
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }

            Ok((String::from(""), String::from("")))
        });

        let join_handle = tokio::spawn(async move {
            // Wait for either ZMQ message, timeout or cancellation
            select! {
                proof = awaiter => {
                    println!("proof: {:?}", proof);
                    proof.unwrap()
                }
                () = timeout_cancellation_token.cancelled() => {
                    Err(anyhow!("Timeout reached while waiting for a proof"))?
                }
                () = request_cancellation_token.cancelled() => {
                    Ok((String::from(""), String::from("")))
                }
            }
        });

        Ok(join_handle)
    }
}
