import { Actor, HttpAgent } from "@dfinity/agent";
import fetch from "isomorphic-fetch";
import canisterIds from "../.dfx/local/canister_ids.json";
import { idlFactory  as verity_verifier_idl } from "../src/declarations/verity_verifier/verity_verifier.did.js";
import { identity } from "./identity.ts";

export const createActor = async (canisterId, options) => {
  const agent = new HttpAgent({ ...options?.agentOptions });
  await agent.fetchRootKey();

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(verity_verifier_idl, {
    agent,
    canisterId,
    ...options?.actorOptions,
  });
};

export const verifyVerifierCanister = canisterIds.verity_verifier.local;

export const verifyVerifier = await createActor(verifyVerifierCanister, {
  agentOptions: {
    host: "http://127.0.0.1:4943",
    fetch,
    identity: await identity,
  },
});