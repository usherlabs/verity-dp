import { describe, expect, test } from "vitest";
import { getCanisterCycles, actor } from "./actor";
import * as fixtures from "verity-fixtures";
import { fail } from "node:assert";

const CANISTER_NAME = "ic-verifier";

describe("IC Verifier", () => {
	test("ping", async () => {
		const old_balance = await getCanisterCycles(CANISTER_NAME);
		const result = await actor.ping();
		console.log(
			"ping cycle used:",
			old_balance - (await getCanisterCycles(CANISTER_NAME)),
		);
		expect(result).toBe("Ping");
	});

	test("public_key", async () => {
		const old_balance = await getCanisterCycles(CANISTER_NAME);
		const startTime = Date.now();

		const result = await actor.public_key();

		expect(typeof result).toBe("object");
		expect(Object.keys(result).length).toBe(2);

		expect(Object.keys(result).includes("sec1_pk")).toBeTruthy();
		expect(typeof result["sec1_pk"]).toBe("string");
		expect(result["sec1_pk"].length).toBe(66);

		expect(Object.keys(result).includes("etherum_pk")).toBeTruthy();
		expect(typeof result["etherum_pk"]).toBe("string");
		expect(result["etherum_pk"].length).toBe(40);

		console.log(`Execution time: ${Date.now() - startTime} ms`);
		console.log(
			"public_key cycle used:",
			old_balance - (await getCanisterCycles(CANISTER_NAME)),
		);
	});

	describe("verify_receipt", () => {
		test("Verify a receipt of verification of a request for 32bytes payload", async () => {
			// Start timing execution
			const old_balance = await getCanisterCycles(CANISTER_NAME);
			const startTime = Date.now();

			const result = await actor.verify_receipt(fixtures.receipt.RECEIPT_32B);

			// Ensure that the canister returns an object of type Result<VerifyReceiptReply, Error>
			expect(typeof result).toBe("object");
			expect(Object.keys(result).length).toBe(1);
			expect(Object.keys(result)[0]).toMatch(/^(Ok|Err)$/);

			// Fail the test if the canister returns an Error
			if (Object.keys(result)[0] === "Err") {
				fail(result["Err"]);
			}

			const reply = result["Ok"];
			expect(typeof reply).toBe("object");

			// Data
			expect(Object.keys(reply).includes("data")).toBeTruthy();
			expect(reply["data"] instanceof Uint8Array).toBeTruthy();

			// Signature
			expect(Object.keys(reply).includes("signature")).toBeTruthy();
			expect(typeof reply["signature"]).toBe("string");
			expect(reply["signature"].length).toBe(130);

			console.log(`Execution time: ${Date.now() - startTime} ms`);
			console.log(
				"verify_receipt cycle used:",
				old_balance - (await getCanisterCycles(CANISTER_NAME)),
			);
		});

		test("Verify a receipt of verification of a request for 1Kb payload", async () => {
			// Start timing execution
			const old_balance = await getCanisterCycles(CANISTER_NAME);
			const startTime = Date.now();

			const result = await actor.verify_receipt(fixtures.receipt.RECEIPT_1KB);

			// Ensure that the canister returns an object of type Result<VerifyReceiptReply, Error>
			expect(typeof result).toBe("object");
			expect(Object.keys(result).length).toBe(1);
			expect(Object.keys(result)[0]).toMatch(/^(Ok|Err)$/);

			// Fail the test if the canister returns an Error
			if (Object.keys(result)[0] === "Err") {
				fail(result["Err"]);
			}

			const reply = result["Ok"];
			expect(typeof reply).toBe("object");

			// Data
			expect(Object.keys(reply).includes("data")).toBeTruthy();
			expect(reply["data"] instanceof Uint8Array).toBeTruthy();

			// Signature
			expect(Object.keys(reply).includes("signature")).toBeTruthy();
			expect(typeof reply["signature"]).toBe("string");
			expect(reply["signature"].length).toBe(130);

			console.log(`Execution time: ${Date.now() - startTime} ms`);
			console.log(
				"verify_receipt cycle used:",
				old_balance - (await getCanisterCycles(CANISTER_NAME)),
			);
		});
	});
});
