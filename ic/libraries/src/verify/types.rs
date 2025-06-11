use anyhow::Result;
use candid::CandidType;
use httparse;
use serde::Deserialize;
use std::{collections::HashMap, vec};

/// The response from the managed verifier canister.
/// It is a `Result` type that contains either a `VerificationResponse` on success
/// or a `String` error message on failure.
pub type VerificationCanisterResponse = Result<VerificationResponse, String>;

/// Represents the response from the managed verifier canister.
/// It includes the proofs, a Merkle root derived from these proofs,
/// and the canister's ECDSA signature of the Merkle root.
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VerificationResponse {
    /// A vector of `ProofResponse` which indicates the source of each proof.
    pub results: Vec<ProofResponse>,
    /// The Merkle root encoded in hexadecimal format.
    pub root: String,
    /// The ECDSA signature of the Merkle root.
    pub signature: String,
}

/// Represents a proof verified by the managed verifier.
/// It can be either a `SessionProof` or a `FullProof`.
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProofResponse {
    FullProof(String),
}

// Implementations for the `ProofResponse` enum.
impl ProofResponse {
    /// Parses the HTTP response and extracts the JSON response body.
    /// This is applicable only for `FullProof`.
    pub fn get_http_response_body(&self) -> String {
        match self {
            ProofResponse::FullProof(text) => {
                let http_parts: Vec<&str> = text.split("\n\n").filter(|s| !s.is_empty()).collect();

                // if empty body and empty request is returned
                if http_parts.len() == 2 {
                    return "".to_string();
                }

                if http_parts[0].contains("application/json") {
                    if let Some(start) = http_parts[1].find('{') {
                        if let Some(end) = http_parts[1].rfind('}') {
                            return http_parts[1][start..=end].to_string();
                        }
                    }
                }

                http_parts[1].to_string()
            }
        }
    }

    /// Parses the HTTP response and extracts the JSON response body.
    /// This is applicable only for `FullProof`.
    pub fn get_http_headers(&self) -> Result<HashMap<String, String>, String> {
        match self {
            ProofResponse::FullProof(text) => {
                // if empty body and empty request is returned
                let response_bytes = text.as_bytes();

                // Prepare space for headers
                let mut headers = [httparse::EMPTY_HEADER; 64]; //64 max response header count
                let mut parsed_response = httparse::Response::new(&mut headers);

                // Parse the response
                match parsed_response.parse(response_bytes) {
                    Ok(httparse::Status::Complete(_header_length)) => {
                        let mut result = HashMap::new();

                        // Insert status code
                        if let Some(code) = parsed_response.code {
                            result.insert("Status".to_string(), code.to_string());
                        }

                        // Insert headers into HashMap
                        for header in parsed_response.headers.iter() {
                            if let Ok(value) = std::str::from_utf8(header.value) {
                                result.insert(header.name.to_string(), value.to_string());
                            }
                        }

                        Ok(result)
                    }
                    Ok(httparse::Status::Partial) => Err("Incomplete HTTP response".to_string()),
                    Err(e) => Err(format!("Failed to parse HTTP response: {:?}", e)),
                }
            }
        }
    }

    /// Retrieves the text content of a verified proof.
    pub fn get_content(&self) -> String {
        match self {
            // For a full proof, return the request/response pair.
            ProofResponse::FullProof(content) => content.clone(),
        }
    }
}

#[cfg(test)]
mod type_test {
    use super::*;

    // Define shared test cases to avoid duplication
    const TEST_CASES: &[(&str, &str, &str)] = &[
        (
            r#"HTTP/1.1 200 OK
Date: Mon, 10 Feb 2025 23:41:20 GMT
Content-Type: application/json; charset=utf-8
Transfer-Encoding: chunked
Connection: close
x-frame-options: SAMEORIGIN
x-xss-protection: 0
x-content-type-options: nosniff
x-download-options: noopen
x-permitted-cross-domain-policies: none
referrer-policy: strict-origin-when-cross-origin
Cache-Control: max-age=30, public, must-revalidate, s-maxage=60
access-control-allow-origin: *
access-control-allow-methods: POST, PUT, DELETE, GET, OPTIONS
access-control-request-method: *
access-control-allow-headers: Origin, X-Requested-With, Content-Type, Accept, Authorization
access-control-expose-headers: link, per-page, total
vary: Accept-Encoding, Origin
etag: W/"0ee2a19705d3e04620b854107a21117b"
x-request-id: 37a1658b-b4a7-4020-9b78-151be90b461c
x-runtime: 0.003170
alternate-protocol: 443:npn-spdy/2
strict-transport-security: max-age=15724800; includeSubdomains
CF-Cache-Status: HIT
Age: 120
Server: cloudflare
CF-RAY: 90fff30a9c833e9a-CPT
alt-svc: h3=":443"; ma=86400

19
{"bitcoin":{"usd":97334}}
0

GET https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd HTTP/1.1
host: api.coingecko.com
accept: */*
cache-control: no-cache
connection: close
accept-encoding: identity
content-type: application/json
x-api-key: XXXXXX

"#,
            r#"{"bitcoin":{"usd":97334}}"#,
            "200",
        ),
        (
            r#"HTTP/1.1 200 OK
Date: Mon, 10 Feb 2025 23:56:29 GMT
Content-Type: application/json
Content-Length: 779
Connection: close
openai-version: 2020-10-01
x-request-id: 5d83f65c8716a7a2d5316861b158eacb
openai-processing-ms: 231
strict-transport-security: max-age=31536000; includeSubDomains; preload
CF-Cache-Status: DYNAMIC
Set-Cookie: __cf_bm=xwU9OwdycTvf_iUGRMeHou.R4aMwygLNo9n8QOMpu9Q-1739231789-1.0.1.1-dn0KFyPa7LEPt4Kvh9wrlpkzyiTsLKqgpowiGWNqLQCvuCIj0U5pTtvqVwSo6i3JRXe4n0y87XGAL1nHZ7ctYg; path=/; expires=Tue, 11-Feb-25 00:26:29 GMT; domain=.api.openai.com; HttpOnly; Secure
X-Content-Type-Options: nosniff
Set-Cookie: _cfuvid=mHKpBMPOySOFR4Ddm2eyW00LQC13wBIguaVCkLhXkGg-1739231789707-0.0.1.1-604800000; path=/; domain=.api.openai.com; HttpOnly; Secure; SameSite=None
Server: cloudflare
CF-RAY: 9100093abb6206cf-CPT
alt-svc: h3=":443"; ma=86400

{
"object": "list",
"data": [
    {
    "id": "o1-mini-2024-09-12",
    "object": "model",
    "created": 1725648979,
    "owned_by": "system"
    },
    {
    "id": "o1-mini",
    "object": "model",
    "created": 1725649008,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o",
    "object": "model",
    "created": 1715367049,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o-mini",
    "object": "model",
    "created": 1721172741,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o-2024-08-06",
    "object": "model",
    "created": 1722814719,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o-mini-2024-07-18",
    "object": "model",
    "created": 1721172717,
    "owned_by": "system"
    }
]
}

GET https://api.openai.com/v1/models HTTP/1.1
host: api.openai.com
accept: */*
cache-control: no-cache
connection: close
accept-encoding: identity
content-type: application/json
authorization: XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
"#,
            r#"{
"object": "list",
"data": [
    {
    "id": "o1-mini-2024-09-12",
    "object": "model",
    "created": 1725648979,
    "owned_by": "system"
    },
    {
    "id": "o1-mini",
    "object": "model",
    "created": 1725649008,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o",
    "object": "model",
    "created": 1715367049,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o-mini",
    "object": "model",
    "created": 1721172741,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o-2024-08-06",
    "object": "model",
    "created": 1722814719,
    "owned_by": "system"
    },
    {
    "id": "gpt-4o-mini-2024-07-18",
    "object": "model",
    "created": 1721172717,
    "owned_by": "system"
    }
]
}"#,
            "200",
        ),
        (
            r#"
HTTP/1.1 201 Created
Date: Fri, 21 Jun 2024 12:35:32 GMT
Content-Type: application/json; charset=utf-8
Content-Length: 69
Connection: close
Report-To: {"group":"heroku-nel","max_age":3600,"endpoints":[{"url":"https://nel.heroku.com/reports?ts=1718973332&sid=e11707d5-02a7-43ef-b45e-2cf4d2036f7d&s=ho9mNnYMVvORRQ3gpBnbquUgUKERGKIM6Bu5rk5iTUc%3D"}]}
Reporting-Endpoints: heroku-nel=https://nel.heroku.com/reports?ts=1718973332&sid=e11707d5-02a7-43ef-b45e-2cf4d2036f7d&s=ho9mNnYMVvORRQ3gpBnbquUgUKERGKIM6Bu5rk5iTUc%3D
Nel: {"report_to":"heroku-nel","max_age":3600,"success_fraction":0.005,"failure_fraction":0.05,"response_headers":["Via"]}
X-Powered-By: Express
X-Ratelimit-Limit: XX00
X-Ratelimit-Remaining: 999
X-Ratelimit-Reset: 1718973343
Vary: Origin, X-HTTP-Method-Override, Accept-Encoding
Access-Control-Allow-Credentials: true
Cache-Control: no-cache
Pragma: no-cache
Expires: -1
Access-Control-Expose-Headers: Location
Location: https://jsonplaceholder.typicode.com/posts/XX1
X-Content-Type-Options: nosniff
Etag: W/"45-5wdvLX9Ar1ABpfyCTUArSzQ3wRo"
Via: 1.1 vegur
CF-Cache-Status: DYNAMIC
Server: cloudflare
CF-RAY: 897409fb9a6d1963-FRA
alt-svc: h3=":443"; ma=86400

{
"title": "usher",
"body": "labs",
"userId": XX,
"id": XX1
}

POST https://jsonplaceholder.typicode.com/posts HTTP/1.1
host: jsonplaceholder.typicode.com
accept: */*
cache-control: no-cache
connection: close
accept-encoding: identity
x-api-key: XXXXXX
content-type: application/json
content-length: 48

{"title": "usher", "body": "labs", "userId": XX}
"#,
            r#"{
"title": "usher",
"body": "labs",
"userId": XX,
"id": XX1
}"#,
            "201",
        ),
        (
            r#"
HTTP/1.1 200 OK
Date: Mon, 10 Feb 2025 23:36:49 GMT
Content-Type: application/json; charset=utf-8
Transfer-Encoding: chunked
Connection: close
x-frame-options: SAMEORIGIN
x-xss-protection: 0
x-content-type-options: nosniff
x-download-options: noopen
x-permitted-cross-domain-policies: none
referrer-policy: strict-origin-when-cross-origin
Cache-Control: max-age=30, public, must-revalidate, s-maxage=60
access-control-allow-origin: *
access-control-allow-methods: POST, PUT, DELETE, GET, OPTIONS
access-control-request-method: *
access-control-allow-headers: Origin, X-Requested-With, Content-Type, Accept, Authorization
access-control-expose-headers: link, per-page, total
vary: Accept-Encoding, Origin
etag: W/"ea5505add7985de260a3f98be206f64c"
x-request-id: 9863f7a3-657d-4ab6-b06b-62aeb8f5aa8c
x-runtime: 0.003897
alternate-protocol: 443:npn-spdy/2
strict-transport-security: max-age=15724800; includeSubdomains
CF-Cache-Status: HIT
Age: 216
Server: cloudflare
CF-RAY: 90ffec6a999e0710-CPT
alt-svc: h3=":443"; ma=86400

19
{"bitcoin":{"usd":97281}}
0

GET https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd HTTP/1.1
host: api.coingecko.com
accept: */*
cache-control: no-cache
connection: close
accept-encoding: identity
content-type: application/json
x-api-key: XXXXXX
content-length: 48
"#,
            r#"{"bitcoin":{"usd":97281}}"#,
            "200",
        ),
    ];

    #[test]
    fn test_verification_canister_response_success() {
        let proof = ProofResponse::FullProof("hashed_content".to_string());
        let verification_response = VerificationResponse {
            results: vec![proof.clone()],
            root: "abcd1234".to_string(),
            signature: "signature1234".to_string(),
        };
        let response: VerificationCanisterResponse = Ok(verification_response.clone());

        assert!(response.is_ok());
        let res = response.unwrap();
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.root, "abcd1234");
        assert_eq!(res.signature, "signature1234");
    }

    #[test]
    fn test_verification_canister_response_failure() {
        let response: VerificationCanisterResponse = Err("error_message".to_string());

        assert!(response.is_err());
        let err = response.unwrap_err();
        assert_eq!(err, "error_message");
    }

    #[test]
    fn test_proof_response_get_http_response_body_full_proof_json() {
        for (input, expected, _) in TEST_CASES {
            let proof = ProofResponse::FullProof(input.to_string());
            let http_body = proof.get_http_response_body();
            assert_eq!(http_body, *expected);
        }
    }

    #[test]
    fn test_proof_response_get_http_response_header_full_proof_json() {
        for (input, _expected, status_code) in TEST_CASES {
            let proof = ProofResponse::FullProof(input.to_string());
            let headers_result = proof.get_http_headers();
            assert!(
                headers_result.is_ok(),
                "Failed to parse headers for input: {}",
                input
            );
            let headers = headers_result.unwrap();

            // Example assertions for specific headers
            assert!(
                headers
                    .get("Content-Type")
                    .unwrap()
                    .contains("application/json"),
                "Content-Type header mismatch for input: {}",
                input
            );
            assert_eq!(headers.get("Status").unwrap(), status_code);
        }
    }
}
