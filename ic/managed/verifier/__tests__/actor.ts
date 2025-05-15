import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { Actor, HttpAgent } from "@dfinity/agent";
import fetch from "isomorphic-fetch";
import { idlFactory as verity_verifier_idl } from "../src/declarations/verity_verifier/verity_verifier.did.js";
import { identity } from "./identity.ts";

let canisterIds: Record<string, any> = {};
const idsPath = path.resolve(__dirname, "../.dfx/local/canister_ids.json");
if (fs.existsSync(idsPath)) {
  try {
    canisterIds = JSON.parse(fs.readFileSync(idsPath, "utf-8"));
  } catch (e) {
    console.warn("Failed to parse canister_ids.json:", e);
  }
} else {
  console.info("No canister_ids.json found, proceeding without it.");
}

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

export const createActor = async (
  canisterId: string,
  options?: {
    agentOptions?: ConstructorParameters<typeof HttpAgent>[0];
    actorOptions?: Omit<Parameters<typeof Actor.createActor>[1], "agent" | "canisterId">;
  },
) => {
  const agent = new HttpAgent({ ...options?.agentOptions });
  await agent.fetchRootKey();
  return Actor.createActor(verity_verifier_idl, {
    agent,
    canisterId,
    ...options?.actorOptions,
  });
};

const is_production = process.env.PROD?.toString() === "true";
const DEV_CANISTER_ID = canisterIds.verity_verifier?.local ?? "";
const PROD_CANISTER_ID = "yf57k-fyaaa-aaaaj-azw2a-cai";

export const verifyVerifierCanister = is_production ? PROD_CANISTER_ID : DEV_CANISTER_ID;

if (!verifyVerifierCanister) {
  console.warn(`No canister ID for ${is_production ? "production" : "development"} build—createActor may fail.`);
}

export const verifyVerifier = await createActor(verifyVerifierCanister, {
  agentOptions: {
    host: is_production ? "https://icp0.io" : "http://127.0.0.1:4943",
    fetch,
    identity: await identity,
  },
});
