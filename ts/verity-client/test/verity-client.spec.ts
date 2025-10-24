/// <reference types="jest" />
import axios from "axios";

// Local ambient declarations for TS type-checkers that don't load Jest globals
// These are safe because Jest provides them at runtime.
declare const jest: any;
declare const describe: any;
declare const test: any;
declare const expect: any;
declare const beforeEach: any;
declare const afterEach: any;

// Mock EventSource module BEFORE importing the client
// Keep all mock state inside the factory (no out-of-scope variables)
jest.mock("eventsource", () => {
  const created: any[] = [];
  // Expose handle for tests via globalThis
  (globalThis as any).__mockEventSources = created;
  return class MockEventSource {
    url: string;
    onopen: ((e: any) => void) | null = null;
    onmessage: ((e: any) => void) | null = null;
    onerror: ((e: any) => void) | null = null;
    constructor(url: string) {
      this.url = url;
      created.push(this);
    }
    close() {}
  };
});

// Import after mocks
import { VerityClient } from "../src";

// Minimal fake Axios instance with interceptor support
function createFakeAxiosInstance(baseURL?: string) {
  const requestHandlers: Array<(c: any) => any | Promise<any>> = [];
  const responseHandlers: Array<(r: any) => any | Promise<any>> = [];
  return {
    defaults: { baseURL },
    interceptors: {
      request: { use: (fn: any) => requestHandlers.push(fn) },
      response: { use: (fn: any) => responseHandlers.push(fn) },
    },
    async request(config: any) {
      for (const h of requestHandlers) config = await h(config);
      // Simulated base response from Prover /proxy
      let response: any = {
        status: config.method === "post" ? 201 : 200,
        statusText: "OK",
        data: config.data ?? { ok: true },
        headers: new Map<string, string>([["t-proof-id", "proof-123"]]),
        config,
      };
      for (const h of responseHandlers) response = await h(response);
      return response;
    },
    get: jest.fn().mockResolvedValue({
      status: 200,
      data: {
        version: "1.0",
        publicKey: "-----BEGIN PUBLIC KEY---xxx",
        gitCommitHash: "a".repeat(40),
        gitCommitTimestamp: "2025-01-01T00:00:00Z",
      },
    }),
  };
}

describe("VerityClient (isolated with mocked Prover)", () => {
  beforeEach(() => {
    const arr = (globalThis as any).__mockEventSources as any[];
    if (arr) arr.length = 0;
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.runOnlyPendingTimers();
    jest.useRealTimers();
    jest.restoreAllMocks();
  });

  test("get_notary_info returns mocked notary details", async () => {
    const baseAxios = createFakeAxiosInstance("http://prover");
    const reqAxios = createFakeAxiosInstance();
    jest
      .spyOn(axios as any, "create")
      .mockReturnValueOnce(baseAxios as any)
      .mockReturnValueOnce(reqAxios as any);

    const client = new VerityClient({ prover_url: "http://prover" });
    const info = await client.get_notary_info();
    expect(info.gitCommitHash).toHaveLength(40);
    expect(info.publicKey.startsWith("-----BEGIN PUBLIC KEY")).toBeTruthy();
  });

  test("GET stitches proof from SSE and sets headers", async () => {
    const baseAxios = createFakeAxiosInstance("http://prover");
    const reqAxios = createFakeAxiosInstance();
    jest
      .spyOn(axios as any, "create")
      .mockReturnValueOnce(baseAxios as any)
      .mockReturnValueOnce(reqAxios as any);

    const client = new VerityClient({ prover_url: "http://prover" });
    const promise = client
      .get(
        "https://api.example.com/data",
        { headers: { "X-TEST": "1" } },
        {
          proofTimeout: 5_000,
        },
      )
      .redact("res:body:id");

    // Simulate SSE lifecycle
    const created = (globalThis as any).__mockEventSources as any[];
    expect(created.length).toBe(1);
    const es = created[0];
    es.onopen?.({}); // mark SSE ready
    // allow request interceptor waitFor() to tick past the first interval
    jest.advanceTimersByTime(100);
    // Deliver proof
    es.onmessage?.({ data: "PUBKEY|PROOF_DATA" });

    const res: any = await promise;
    expect(res.status).toBe(200);
    expect(res.notary_pub_key).toBe("PUBKEY");
    expect(res.proof).toBe("PROOF_DATA");
    // Headers applied by interceptor
    expect(res.config.headers["T-REQUEST-ID"]).toBeDefined();
    expect(res.config.headers["T-PROXY-URL"]).toBe("https://api.example.com/data");
    // Simulated base header retained
    expect(res.headers.get ? res.headers.get("t-proof-id") : res.headers.has("t-proof-id")).toBeTruthy();
  });

  test("SSE error resolves empty proof when throwOnError=false", async () => {
    const baseAxios = createFakeAxiosInstance("http://prover");
    const reqAxios = createFakeAxiosInstance();
    jest
      .spyOn(axios as any, "create")
      .mockReturnValueOnce(baseAxios as any)
      .mockReturnValueOnce(reqAxios as any);

    const client = new VerityClient({ prover_url: "http://prover" });
    const promise = client.get(
      "https://api.example.com/data",
      {},
      {
        throwOnError: false,
        proofTimeout: 5_000,
      },
    );

    const es = ((globalThis as any).__mockEventSources as any[])[0];
    es.onopen?.({});
    // unblock interceptor waitFor
    jest.advanceTimersByTime(100);
    es.onerror?.({ message: "boom" } as any);

    const res: any = await promise;
    expect(res.status).toBe(200);
    expect(res.proof ?? "").toBe("");
  });

  test("SSE timeout uses proofTimeout and yields empty proof when throwOnError=false", async () => {
    const baseAxios = createFakeAxiosInstance("http://prover");
    const reqAxios = createFakeAxiosInstance();
    jest
      .spyOn(axios as any, "create")
      .mockReturnValueOnce(baseAxios as any)
      .mockReturnValueOnce(reqAxios as any);

    const client = new VerityClient({ prover_url: "http://prover" });
    const promise = client.get("https://api.example.com/data", {}, { proofTimeout: 10 });

    const es = ((globalThis as any).__mockEventSources as any[])[0];
    es.onopen?.({});
    // unblock interceptor waitFor then trigger SSE timeout
    jest.advanceTimersByTime(100);
    jest.advanceTimersByTime(11);

    const res: any = await promise;
    expect(res.proof ?? "").toBe("");
  });

  test("propagates SSE error when throwOnError=true", async () => {
    const baseAxios = createFakeAxiosInstance("http://prover");
    const reqAxios = createFakeAxiosInstance();
    jest
      .spyOn(axios as any, "create")
      .mockReturnValueOnce(baseAxios as any)
      .mockReturnValueOnce(reqAxios as any);

    const client = new VerityClient({ prover_url: "http://prover" });
    const promise = client.get(
      "https://api.example.com/data",
      {},
      {
        throwOnError: true,
        proofTimeout: 5_000,
      },
    );

    const es = ((globalThis as any).__mockEventSources as any[])[0];
    es.onopen?.({});
    // Unblock interceptor waitFor
    jest.advanceTimersByTime(100);
    es.onerror?.({ message: "boom" } as any);

    await expect(promise).rejects.toBeTruthy();
  });
});
