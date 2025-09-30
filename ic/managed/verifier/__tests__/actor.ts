import { execSync } from "node:child_process";
import canisterIds from "../.dfx/local/canister_ids.json";
import { createActor } from "../src/declarations/verity_verifier/index.js";
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

const is_production = process.env.PROD?.toString() === "true";
const DEV_CANISTER_ID = canisterIds.verity_verifier?.local ?? "";
const PROD_CANISTER_ID = "yf57k-fyaaa-aaaaj-azw2a-cai";

export const verityVerifierCanister = is_production ? PROD_CANISTER_ID : DEV_CANISTER_ID;

if (!verityVerifierCanister) {
  console.warn(`No canister ID for ${is_production ? "production" : "development"} build—createActor may fail.`);
}

export const actor = await createActor(verityVerifierCanister, {
  agentOptions: {
    host: is_production ? "https://icp0.io" : "http://127.0.0.1:4943",
    fetch,
    identity: await identity,
  },
});
