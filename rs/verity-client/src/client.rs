use std::{future::IntoFuture, str::FromStr};

use http::{HeaderValue, Method};
use reqwest::{IntoUrl, Response, Url};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::request::RequestBuilder;
use crate::Error;

#[derive(Clone)]
pub struct VerityClientConfig {
    pub prover_url: String,
    pub prover_zmq: String,
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

        let proof_future = async { self.receive_proof(request_id.to_string()).await };

        let (response, proof) = tokio::join!(req.send(), proof_future);
        let subject = match response {
            Ok(response) => response,
            Err(e) => return Err(Error::Reqwest(e)),
        };

        let proof = match proof {
            Ok(proof) => proof,
            Err(e) => return Err(Error::Verity(e.into())),
        };

        let (notary_pub_key, proof) = proof;

        Ok(VerityResponse {
            subject,
            proof,
            notary_pub_key,
        })
    }

    fn receive_proof(&self, request_id: String) -> JoinHandle<(String, String)> {
        let prover_zmq = self.config.prover_zmq.clone();

        tokio::task::spawn_blocking(move || {
            let context = zmq::Context::new();
            let subscriber = context.socket(zmq::SUB).unwrap();
            assert!(subscriber.connect(prover_zmq.as_str()).is_ok());
            assert!(subscriber.set_subscribe(request_id.as_bytes()).is_ok());

            let proof = subscriber.recv_string(0).unwrap().unwrap();

            // TODO: Gracefully shutdown the ZMQ subscriber with the context
            subscriber.set_unsubscribe(b"").unwrap();

            // TODO: Better split session_id and the proof. See multipart ZMQ messaging.
            let parts: Vec<&str> = proof.splitn(4, "|").collect();

            (parts[1].to_string(), parts[2].to_string())
        })
        .into_future()
    }
}
