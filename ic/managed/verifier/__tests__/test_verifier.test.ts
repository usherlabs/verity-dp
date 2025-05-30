import { readFileSync, readdirSync } from "node:fs";
import { describe, expect, test } from "vitest";
import type { ProofVerificationResponse } from "../src/declarations/verity_verifier/verity_verifier.did";
import { getCanisterCycles, verifyVerifier } from "./actor";

const test_files = readdirSync("./fixtures/")
  .filter((file) => file.endsWith(".json"))
  .map((elem) => readFileSync(`./fixtures/${elem}`, "utf8"));
const public_key = readFileSync("./fixtures/notary.pub", "utf8");

describe("Managed IC Verifier", () => {
  test("expect Ping", async () => {
    const old_balance = await getCanisterCycles("verity_verifier");
    const result = await verifyVerifier.ping();
    console.log("Ping cycle used:", old_balance - (await getCanisterCycles("verity_verifier")));
    expect(result).toBe("Ping");
  });

  describe("Process verify_proof_async", () => {
    test("Expect verify_proof_async to return Proof Response", async () => {
      // Start timing execution
      const old_balance = await getCanisterCycles("verity_verifier");
      const startTime = Date.now();
      console.log({ length: test_files.reduce((prev, curr) => prev + curr.length, 0) });
      const result = (await verifyVerifier.verify_proof_async(test_files, public_key)) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(true);
      expect(result.length).toBe(test_files.length);

      for (const item of result) {
        expect(typeof item).toBe("object");
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe("string");
      }
      console.log(`Execution time: ${Date.now() - startTime} ms`);
      console.log("verify_proof_async cycle used:", old_balance - (await getCanisterCycles("verity_verifier")));
    });

    test("Expect verify_proof_async_batch to return Proof Response", async () => {
      const old_balance = await getCanisterCycles("verity_verifier");
      const startTime = Date.now();
      const result = (await verifyVerifier.verify_proof_async_batch([
        {
          proof_requests: test_files.slice(0, test_files.length / 2),
          notary_pub_key: public_key,
        },
        {
          proof_requests: test_files.slice(test_files.length / 2),
          notary_pub_key: public_key,
        },
      ])) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(true);
      expect(result.length).toBe(test_files.length);
      for (const item of result) {
        expect(typeof item).toBe("object");
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe("string");
      }
      console.log(`Execution time: ${Date.now() - startTime} ms`);
      console.log("verify_proof_async Batch cycle used:", old_balance - (await getCanisterCycles("verity_verifier")));
    }, 1000000);
  });

  describe("Process verify_proof_direct", () => {
    test("Expect verify_proof_direct for a single notary to return Proof Response", async () => {
      const old_balance = await getCanisterCycles("verity_verifier");
      const startTime = Date.now();

      const result = (await verifyVerifier.verify_proof_direct(test_files, public_key)) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(false);
      expect(Object.keys(result)).toMatch(/^(Ok|Err)$/);

      expect(result).toHaveProperty("Ok");
      for (const item of result["Ok"].results) {
        expect(typeof item).toBe("object");
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe("string");
      }
      expect(typeof result["Ok"]["signature"]).toBe("string");
      expect(typeof result["Ok"]["root"]).toBe("string");
      console.log(`Execution time: ${Date.now() - startTime} ms`);
      console.log("verify_proof_direct cycle used:", old_balance - (await getCanisterCycles("verity_verifier")));
    }, 1000000);

    test("Expect verify_proof_direct for a batch to return Proof Response", async () => {
      const old_balance = await getCanisterCycles("verity_verifier");
      const startTime = Date.now();

      const result = (await verifyVerifier.verify_proof_direct_batch([
        {
          proof_requests: test_files.slice(0, test_files.length / 2),
          notary_pub_key: public_key,
        },
        {
          proof_requests: test_files.slice(test_files.length / 2),
          notary_pub_key: public_key,
        },
      ])) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(false);
      expect(Object.keys(result)).toMatch(/^(Ok|Err)$/);

      expect(result).toHaveProperty("Ok");
      for (const item of result["Ok"].results) {
        expect(typeof item).toBe("object");
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe("string");
      }

      console.log(`Execution time: ${Date.now() - startTime} ms`);
      console.log("verify_proof_direct batch cycle used:", old_balance - (await getCanisterCycles("verity_verifier")));
      expect(typeof result["Ok"]["signature"]).toBe("string");
      expect(typeof result["Ok"]["root"]).toBe("string");
    }, 1000000);
  });
});
