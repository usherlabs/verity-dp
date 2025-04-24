import { execSync } from "node:child_process";
import { Actor, HttpAgent } from "@dfinity/agent";
import fetch from "isomorphic-fetch";
import canisterIds from "../.dfx/local/canister_ids.json";
import { idlFactory as verity_verifier_idl } from "../src/declarations/verity_verifier/verity_verifier.did.js";
import { identity } from "./identity.ts";

export function getCanisterCycles(canisterName: string): number {
  try {
    const statusOutput = execSync(`dfx canister status ${canisterName}`, { encoding: "utf-8" });
    const match = statusOutput.match(/Balance:\s+([\d_]+)/);
    if (match) {
      return Number.parseInt(match[1].replace(/_/g, ""), 10);
    }
  } catch (error) {
    console.error(`Error fetching canister cycles: ${error}`);
  }
  return 0;
}

export const createActor = async (canisterId, options) => {
  const agent = new HttpAgent({ ...options?.agentOptions });
  const x = await agent.fetchRootKey();

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(verity_verifier_idl, {
    agent,
    canisterId,
    ...options?.actorOptions,
  });
};
const is_production = process.env.PROD?.toString() === "true";

export const verifyVerifierCanister = is_production ? "yf57k-fyaaa-aaaaj-azw2a-cai" : canisterIds.verity_verifier.local;

export const verifyVerifier = await createActor(verifyVerifierCanister, {
  agentOptions: {
    host: is_production ? "https://icp0.io" : "http://127.0.0.1:4943",
    fetch,
    identity: await identity,
  },
});
