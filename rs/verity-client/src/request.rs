use std::convert::TryFrom;

use http::{HeaderName, HeaderValue};
use reqwest::{header::HeaderMap, Body, Request};
use serde::Serialize;

use crate::client::{VerityClient, VerityResponse};

/// A builder to construct the properties of a `Request`.
///
/// To construct a `RequestBuilder`, refer to the `Client` documentation.
#[must_use = "RequestBuilder does nothing until you 'send' it"]
pub struct RequestBuilder {
    pub(crate) client: VerityClient,
    pub(crate) inner: reqwest::RequestBuilder,
}

impl RequestBuilder {
    /// Add a `Header` to this Request.
    ///
    /// This method allows you to add a single header to the request.
    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            inner: self.inner.header(key, value),
            ..self
        }
    }

    /// Add a set of Headers to the existing ones on this Request.
    ///
    /// This method merges the provided headers with any already set on the request.
    pub fn headers(self, headers: HeaderMap) -> Self {
        RequestBuilder {
            inner: self.inner.headers(headers),
            ..self
        }
    }

    /// Set the request body.
    ///
    /// This method sets the body of the request to the provided value.
    pub fn body<T: Into<Body>>(self, body: T) -> Self {
        RequestBuilder {
            inner: self.inner.body(body),
            ..self
        }
    }

    /// Send a JSON body.
    ///
    /// This method serializes the provided data structure as JSON and sets it as the request body.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    pub fn json<T: Serialize + ?Sized>(self, json: &T) -> Self {
        RequestBuilder {
            inner: self.inner.json(json),
            ..self
        }
    }

    /// Add an instruction to prove failed request.
    ///
    /// This method adds a header to instruct Verity Prover to prove the response,
    /// even if its status code is not success.
    pub fn prove_failed_request(self) -> Self {
        RequestBuilder {
            inner: self.inner.header("T-PROVE-FAILED-REQ", "true"),
            ..self
        }
    }

    /// Add a Redact instruction.
    ///
    /// This method adds a header to instruct Verity Prover on how to hide sensitive data.
    pub fn redact(self, redact: String) -> Self {
        RequestBuilder {
            inner: self
                .inner
                .header("T-REDACTED", HeaderValue::from_str(&redact).unwrap()),
            ..self
        }
    }

    /// Build a `Request`.
    ///
    /// This method constructs the request, which can then be
    /// inspected, modified and executed with `VerityClient::execute()`.
    pub fn build(self) -> reqwest::Result<Request> {
        self.inner.build()
    }

    /// Build a `Request`, which can be inspected, modified and executed with
    /// `VerityClient::execute()`.
    ///
    /// This is similar to [`RequestBuilder::build()`], but also returns the
    /// embedded `VerityClient`.
    pub fn build_split(self) -> (VerityClient, reqwest::Result<Request>) {
        let Self { inner, client, .. } = self;
        let (_, req) = inner.build_split();

        (client, req)
    }

    /// Constructs the Request and sends it to the target URL, returning a
    /// future Response.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending request,
    /// redirect loop was detected or redirect limit was exhausted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use anyhow::Error;
    /// # use verity_client::client::{VerityClient, VerityClientConfig};
    /// # async fn run() -> Result<(), Error> {
    ///
    ///
    /// let config = VerityClientConfig {
    ///     prover_url: String::from("http://127.0.0.1:8080"),
    /// };
    ///
    /// let response = VerityClient::new(config)
    ///     .get("https://hyper.rs")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> anyhow::Result<VerityResponse> {
        let (mut client, req) = self.build_split();
        client.execute_request(req?).await
    }
}
