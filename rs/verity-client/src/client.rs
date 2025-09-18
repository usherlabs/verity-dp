use std::str::FromStr;
use std::time::Duration;

use futures::stream::StreamExt;
use http::{HeaderValue, Method};
use reqwest::{IntoUrl, Response, Url};
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::error;
use uuid::Uuid;

use crate::request::RequestBuilder;

/// Time to wait for a proof received over SSE connection since receiving HTTP response
const PROOF_TIMEOUT: Duration = Duration::from_millis(1000);

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaryInformation {
    pub version: String,
    pub public_key: String,
    pub git_commit_hash: String,
}

impl VerityClient {
    /// Creates a new `VerityClient` with the given configuration.
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

    /// Starts building a `Request` with the specified `Method` and `Url`.
    ///
    /// Returns a `RequestBuilder`, which allows setting headers and
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

    /// Executes a `Request` and returns a `VerityResponse`.
    ///
    /// A `Request` can be built manually with `Request::new()` or obtained
    /// from a RequestBuilder with `RequestBuilder::build()`.
    ///
    /// You should prefer to use the `RequestBuilder` and
    /// `RequestBuilder::send()`.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending the request,
    /// a redirect loop was detected, or the redirect limit was exhausted.
    pub async fn execute(&mut self, request: reqwest::Request) -> anyhow::Result<VerityResponse> {
        self.execute_request(request).await
    }

    /// Executes the given request and awaits proof of execution.
    ///
    /// # Errors
    ///
    /// This method fails if the request cannot be sent or if proof cannot be obtained.
    pub async fn execute_request(
        &mut self,
        mut req: reqwest::Request,
    ) -> anyhow::Result<VerityResponse> {
        let proxy_url = &String::from(req.url().as_str());
        let headers = req.headers_mut();

        let request_id = Uuid::new_v4();
        headers.append(
            "T-REQUEST-ID",
            HeaderValue::from_str(&format!("{}", request_id))?,
        );

        headers.append("T-PROXY-URL", HeaderValue::from_str(proxy_url)?);

        *req.url_mut() = Url::from_str(&format!("{}/proxy", self.config.prover_url))?;

        let req = reqwest::RequestBuilder::from_parts(self.inner.clone(), req);

        let request_cancellation_token = CancellationToken::new();
        let timeout_cancellation_token = CancellationToken::new();

        let proof_awaiter = self.await_proof(
            request_id.to_string(),
            request_cancellation_token.clone(),
            timeout_cancellation_token.clone(),
        )?;

        // prettier-ignore
        let (response, proof_msg) = tokio::try_join!(
            self.send_request(req, request_cancellation_token, timeout_cancellation_token),
            proof_awaiter
        )
        .map_err(|e| anyhow::anyhow!("Failed to prove the request: {}", e))?;

        let subject = response?;
        let (notary_pub_key, proof) = proof_msg?;

        Ok(VerityResponse {
            subject,
            proof,
            notary_pub_key,
        })
    }

    /// Sends the request and handles cancellation tokens.
    ///
    /// Returns a `JoinHandle` that resolves to the response or an error.
    fn send_request(
        &self,
        request: reqwest::RequestBuilder,
        request_cancellation_token: CancellationToken,
        timeout_cancellation_token: CancellationToken,
    ) -> JoinHandle<anyhow::Result<reqwest::Response>> {
        tokio::spawn(async move {
            let result = request.send().await;
            let response = result.map_err(|e| {
                error!("{}", e);
                e
            })?;

            // If T-PROOF-ID header has value, wait for the proof with the timeout,
            // otherwise stop waiting
            if response.headers().get("T-PROOF-ID").is_some() {
                tokio::spawn(async move {
                    tokio::time::sleep(PROOF_TIMEOUT).await;
                    timeout_cancellation_token.cancel();
                });
            } else {
                request_cancellation_token.cancel();
                return Ok(response);
            }

            Ok(response)
        })
    }

    /// Awaits proof of request execution.
    ///
    /// Returns a `JoinHandle` that resolves to the proof or an error.
    ///
    /// # Errors
    ///
    /// This method fails if the proof cannot be obtained.
    fn await_proof(
        &self,
        request_id: String,
        request_cancellation_token: CancellationToken,
        timeout_cancellation_token: CancellationToken,
    ) -> anyhow::Result<JoinHandle<anyhow::Result<(String, String)>>> {
        let url = Url::from_str(&format!("{}/proof/{}", self.config.prover_url, request_id))?;
        let mut event_source = EventSource::get(url);

        let awaiter = tokio::task::spawn(async move {
            while let Some(event) = event_source.next().await {
                match event {
                    Ok(Event::Open) => {}
                    Ok(Event::Message(message)) => {
                        let parts: Vec<&str> = message.data.splitn(2, "|").collect();
                        if parts.len() != 2 {
                            anyhow::bail!("Invalid proof response");
                        }

                        return Ok((parts[0].to_string(), parts[1].to_string()));
                    }
                    Err(err) => {
                        error!("{}", err);
                        Err(err)?;
                    }
                }
            }

            Ok((String::from(""), String::from("")))
        });

        let join_handle = tokio::spawn(async move {
            // Wait for either SSE message, timeout or cancellation
            select! {
                proof = awaiter => {
                    proof.unwrap()
                }
                () = timeout_cancellation_token.cancelled() => {
                    anyhow::bail!("Timeout reached while waiting for a proof")
                }
                () = request_cancellation_token.cancelled() => {
                    Ok((String::new(), String::new()))
                }
            }
        });

        Ok(join_handle)
    }

    /// Get the information of the connected notary
    pub async fn get_notary_info(&self) -> anyhow::Result<NotaryInformation> {
        let notary_info_url = format!("{}/notaryinfo", self.config.prover_url);
        let notary_information = reqwest::get(notary_info_url)
            .await?
            .json::<NotaryInformation>()
            .await?;

        Ok(notary_information)
    }
}
