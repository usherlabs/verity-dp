import { execSync } from "node:child_process";
import { identity } from "./identity.ts";
import { createActor } from "../src/declarations/ic-verifier/index.js";
import canisterIds from "../.dfx/local/canister_ids.json";

export function getCanisterCycles(canisterName: string): number {
	try {
		const statusOutput = execSync(`dfx canister status ${canisterName}`, {
			encoding: "utf-8",
		});
		const match = statusOutput.match(/Balance:\s+([\d_]+)/);
		if (match) {
			return Number.parseInt(match[1].replace(/_/g, ""), 10);
		}
	} catch (error) {
		console.error(`Error fetching canister cycles: ${error}`);
	}
	return 0;
}

export const actor = await createActor(canisterIds["ic-verifier"].local, {
	agentOptions: {
		host: "http://127.0.0.1:4943",
		fetch,
		identity: await identity,
	},
});
