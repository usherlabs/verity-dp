import { describe, expect, test, Vitest, vitest } from "vitest";
import { verifyVerifier } from "./actor";
import { readFileSync, readdirSync } from "fs"
import { ProofVerificationResponse } from "../src/declarations/verity_verifier/verity_verifier.did";

const test_files = readdirSync("./fixtures/").filter(file => file.endsWith(".json")).map(elem => readFileSync(`./fixtures/${elem}`, "utf8"))
const public_key = readFileSync("./fixtures/notary.pub", "utf8")


describe("Managed IC Verifier", () => {

  test("expect Ping", async () => {
    const result = await verifyVerifier.ping();
    expect(result).toBe("Ping");
  });

  describe("Process verify_proof_async", () => {

    test("Expect verify_proof_async to return Proof Response", async () => {
      const result = await verifyVerifier.verify_proof_async(test_files, public_key) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(true);
      expect(result.length).toBe(test_files.length);
      result.forEach(item => {
        expect(typeof item).toBe('object');
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe('string');
      });
    });


    test("Expect verify_proof_async_batch to return Proof Response", async () => {
      const result = await verifyVerifier.verify_proof_async_batch([
        {
          'proof_requests': test_files.slice(0, test_files.length / 2),
          'notary_pub_key': public_key
        },
        {
          'proof_requests': test_files.slice(test_files.length / 2),
          'notary_pub_key': public_key
        }
      ]) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(true);
      expect(result.length).toBe(test_files.length);
      result.forEach(item => {
        expect(typeof item).toBe('object');
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe('string');
      });
    });
  })

  describe("Process verify_proof_direct", () => {
    test("Expect verify_proof_direct for a single notary to return Proof Response", async () => {
      const result = await verifyVerifier.verify_proof_direct(test_files, public_key) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(false);
      expect(Object.keys(result)).toMatch(/^(Ok|Err)$/);

      expect(result).toHaveProperty("Ok");
      result["Ok"].results.forEach(item => {
        expect(typeof item).toBe('object');
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe('string');
      });
      expect(typeof result["Ok"]["signature"]).toBe("string")
      expect(typeof result["Ok"]["root"]).toBe("string")
    }, 10000);

    test("Expect verify_proof_direct for a batch to return Proof Response", async () => {
      const result = await verifyVerifier.verify_proof_direct_batch([
        {
          'proof_requests': test_files.slice(0, test_files.length / 2),
          'notary_pub_key': public_key
        },
        {
          'proof_requests': test_files.slice(test_files.length / 2),
          'notary_pub_key': public_key
        }
      ]) as ProofVerificationResponse;
      expect(Array.isArray(result)).toBe(false);
      expect(Object.keys(result)).toMatch(/^(Ok|Err)$/);

      expect(result).toHaveProperty("Ok");
      result["Ok"].results.forEach(item => {
        expect(typeof item).toBe('object');
        expect(Object.keys(item).length).toBe(1);
        expect(Object.keys(item)[0]).toMatch(/^(FullProof|SessionProof)$/);
        expect(typeof Object.values(item)[0]).toBe('string');
      });
      expect(typeof result["Ok"]["signature"]).toBe("string")
      expect(typeof result["Ok"]["root"]).toBe("string")
    }, 10000);
  })

})