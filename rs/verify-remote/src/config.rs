use ic_agent::Agent;
use ic_agent::{export::Principal, identity::Secp256k1Identity};
use serde::{Deserialize, Serialize};

use crate::ic::{DEFAULT_IC_GATEWAY_MAINNET, DEFAULT_IC_GATEWAY_MAINNET_TRAILING_SLASH};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// The URL of the ICP server to connect to
    pub url: String,
    /// The path to the pem keyfile to generate an identity from
    pub keyfile_path: String,
    /// is this dev or prod env
    pub is_dev: bool,
    /// the principal of the calling canister
    pub canister_principal: Principal,
}

impl Config {
    pub fn new(rpc_url: String, keyfile_path: String, verifier_canister_principal: String) -> Self {
        let is_mainnet = matches!(
            &rpc_url[..],
            DEFAULT_IC_GATEWAY_MAINNET | DEFAULT_IC_GATEWAY_MAINNET_TRAILING_SLASH
        );

        Self {
            url: rpc_url,
            keyfile_path,
            canister_principal: Principal::from_text(verifier_canister_principal).unwrap(),
            is_dev: !is_mainnet,
        }
    }

    pub async fn create_agent(&self) -> anyhow::Result<Agent> {
        let identity = Secp256k1Identity::from_pem_file(&self.keyfile_path)?;
        let agent = Agent::builder()
            .with_transport(ic_agent::agent::http_transport::ReqwestTransport::create(
                self.url.clone(),
            )?)
            .with_boxed_identity(Box::new(identity))
            .with_verify_query_signatures(true)
            .build()?;

        if self.is_dev {
            agent.fetch_root_key().await?;
        }

        Ok(agent)
    }
}
