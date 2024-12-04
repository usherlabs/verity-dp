use crate::crypto::ecdsa;
use candid::CandidType;
use serde_derive::Deserialize;

/// Represents the environment in which the canister operates.
#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Environment {
	/// Development environment, used for local testing and development.
	#[default]
	Development,
	/// Staging environment, used for pre-production testing.
	Staging,
	/// Production environment, used for live deployments.
	Production,
}

/// Configuration settings for the canister.
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Config {
	/// The current environment setting.
	pub env: Environment,
	/// ECDSA key identifiers used for cryptographic operations.
	pub key: ecdsa::EcdsaKeyIds,
	/// The number of cycles allocated for signing operations.
	pub sign_cycles: u64,
}

impl Default for Config {
	/// Provides a default configuration, using the Development environment.
	fn default() -> Self {
		Self::from(Environment::Development)
	}
}

impl From<Environment> for Config {
	/// Creates a configuration based on the specified environment.
	fn from(env: Environment) -> Self {
		if env == Environment::Staging {
			Self {
				env: Environment::Staging,
				key: ecdsa::EcdsaKeyIds::TestKey1,
				sign_cycles: 10_000_000_000,
			}
		} else if env == Environment::Production {
			Self {
				env: Environment::Production,
				key: ecdsa::EcdsaKeyIds::ProductionKey1,
				sign_cycles: 26_153_846_153,
			}
		} else {
			Self {
				env: Environment::Development,
				key: ecdsa::EcdsaKeyIds::TestKeyLocalDevelopment,
				sign_cycles: 25_000_000_000,
			}
		}
	}
}
