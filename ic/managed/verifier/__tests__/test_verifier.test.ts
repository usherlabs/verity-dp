import { fail } from "node:assert";
import * as fixtures from "verity-fixtures";
import { describe, expect, test } from "vitest";
import type {
  PayloadBatch,
  PresentationBatch,
  VerificationResponse,
} from "../src/declarations/verity_verifier/verity_verifier.did";
import { actor, getCanisterCycles } from "./actor";

const CANISTER_NAME = "verity_verifier";

describe("Managed Verifier", () => {
  test("ping", async () => {
    const old_balance = await getCanisterCycles(CANISTER_NAME);
    const result = await actor.ping();
    console.log("ping cycle used:", old_balance - (await getCanisterCycles(CANISTER_NAME)));
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
    console.log("public_key cycle used:", old_balance - (await getCanisterCycles(CANISTER_NAME)));
  });

  describe("Process verify_async", () => {
    test("Expect verify_async to return PayloadBatches", async () => {
      // Start timing execution
      const old_balance = await getCanisterCycles("verity_verifier");
      const startTime = Date.now();
      const presentationBatches: PresentationBatch[] = [
        {
          notary_pub_key: fixtures.notary.PUB_KEY,
          presentations: [fixtures.presentation.PRESENTATION_32B, fixtures.presentation.PRESENTATION_1KB],
        },
      ];
      const result = await actor.verify_async(presentationBatches);

      // Ensure that the canister returns an object of type Result<Vec<PayloadBatch>, String>
      expect(typeof result).toBe("object");
      expect(Object.keys(result).length).toBe(1);
      expect(Object.keys(result)[0]).toMatch(/^(Ok|Err)$/);

      // Fail the test if the canister returns an Error
      if ("Err" in result) {
        fail(result.Err);
      }

      const payloadBatches = (result as { Ok: PayloadBatch[] }).Ok;
      expect(Array.isArray(payloadBatches)).toBe(true);
      expect(payloadBatches.length).toBe(presentationBatches.length);

      for (const payloadBatch of payloadBatches) {
        expect(payloadBatch.payloads.length).toBe(2);
        for (const payload of payloadBatch.payloads) {
          expect(payload.sent.length).toBeGreaterThan(0);
          expect(payload.received.length).toBeGreaterThan(0);
        }
      }

      console.log(`Execution time: ${Date.now() - startTime} ms`);
      console.log("verify_proof_async cycle used:", old_balance - (await getCanisterCycles("verity_verifier")));
    });
  });

  describe("Process verify_direct", () => {
    test("Expect verify_direct to return PayloadBatches, Merkle root and Signature", async () => {
      // Start timing execution
      const old_balance = await getCanisterCycles("verity_verifier");
      const startTime = Date.now();
      const presentationBatches: PresentationBatch[] = [
        {
          notary_pub_key: fixtures.notary.PUB_KEY,
          presentations: [fixtures.presentation.PRESENTATION_32B, fixtures.presentation.PRESENTATION_1KB],
        },
      ];
      const result = await actor.verify_direct(presentationBatches);

      // Ensure that the canister returns an object of type Result<Vec<PayloadBatch>, String>
      expect(typeof result).toBe("object");
      expect(Object.keys(result).length).toBe(1);
      expect(Object.keys(result)[0]).toMatch(/^(Ok|Err)$/);

      // Fail the test if the canister returns an Error
      if ("Err" in result) {
        fail(result.Err);
      }

      const response = (result as { Ok: VerificationResponse }).Ok;
      expect(response.payload_batches.length).toBe(presentationBatches.length);

      const payloadBatch = response.payload_batches[0];
      expect(payloadBatch.payloads.length).toBe(2);
      for (const payload of payloadBatch.payloads) {
        expect(payload.sent.length).toBeGreaterThan(0);
        expect(payload.received.length).toBeGreaterThan(0);
      }

      expect(response.root.length).toBe(64);
      expect(response.signature.length).toBe(130);

      console.log(`Execution time: ${Date.now() - startTime} ms`);
      console.log("verify_proof_async cycle used:", old_balance - (await getCanisterCycles("verity_verifier")));
    });
  });
});
