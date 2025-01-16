searchState.loadedDescShard("verity_ic", 0, "This module serves as the main entry point for the …\nThis submodule contains cryptographic related operations.\nThis submodule manages the owner of a canister and provides\nThe ‘random’ submodule provides functionality for …\nThe ‘remittance’ submodule contains logic for the …\nThe ‘whitelist’ module provides CRUD operations for …\nRemoves the leading elements from a vector that match the …\nConverts a hexadecimal string (optionally prefixed with ‘…\nConverts a vector of bytes to a hexadecimal string.\nConfiguration settings for the canister.\nDevelopment environment, used for local testing and …\nRepresents the environment in which the canister operates.\nProduction environment, used for live deployments.\nStaging environment, used for pre-production testing.\nProvides a default configuration, using the Development …\nThe current environment setting.\nReturns the argument unchanged.\nCreates a configuration based on the specified environment.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nECDSA key identifiers used for cryptographic operations.\nThe number of cycles allocated for signing operations.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert a compressed SEC1 public key (33 bytes) to an …\nAppend a recovery identifier byte to the ECDSA signature\nPreprocess and hash an Ethereum message using the Ethereum …\nRecover the Ethereum address from a given signature and …\nSign a message using ECDSA and return the signature\nRetrieves the owner of the canister as a string. Panics if …\nInitializes the owner variable during the canister’s …\nEnsures that the caller of a canister method is the owner. …\nRetrieves a random number for use in the canister. Assumes …\nInitializes the random number generator for the canister. …\nRetrieves the available balance for an account associated …\nRetrieves the canister balance for a specific token and …\nGet the reciept of a successfull withdrawal\nRetrieves the withheld balance for an account on a …\nInitializes the remittance canister state\nReturns the name of the contract\nReturns the owner of the contract\nGet the public key associated with this particular canister\nCreates a remittance request\nSubscribes to a DC canister using its canister_id\nSubscribes to a PCD canister using its canister_id\nValidates and updates user and canister balances\nReturns the argument unchanged.\nGet the remittance canister registered with the DC canister\nCalls <code>U::from(self)</code>.\nChecks if the provided canister principal is already …\nPublish an array of DC state changes to the remittance …\nPublish an array of PDC state changes to the remittance …\nSet a value for the remittance canister registered\nSubscribe to the DC canister\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCancels a withdrawal request, returning the withheld …\nConfirms a successful withdrawal by updating the necessary …\nRetrieves the total available balance for a user.\nRetrieves the total balance available to a specific …\nRetrieves the remitted balance for a given account and …\nHashes the parameters needed for a remittance transaction.\nEnsures the caller is a whitelisted DC canister, otherwise …\nUpdates the balance for a specific account in a DC …\nUpdates the canister’s balance for a specific token.\nValidates remittance data for processing by a DC canister.\nValidates remittance data for processing by a PDC.\nValidates remittance data based on whether the caller is a …\nContains the error value\nContains the success value\nRepresents a proof verified by the managed verifier. It …\nThe response from the managed verifier canister. It is a …\nRepresents the response from the managed verifier canister.\nReturns the argument unchanged.\nReturns the argument unchanged.\nRetrieves the text content of a verified proof.\nParses the HTTP response and extracts the JSON response …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA vector of <code>ProofResponse</code> which indicates the source of …\nThe Merkle root encoded in hexadecimal format.\nThe ECDSA signature of the Merkle root.\nA thread-local storage for the whitelist, mapping …\nAdds a principal to the whitelist.\nChecks if a principal is whitelisted. Returns <code>true</code> if the …\nRemoves a principal from the whitelist.")