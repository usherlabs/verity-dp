import type { AxiosInstance, AxiosRequestConfig, AxiosRequestHeaders, AxiosResponse } from "axios";
import axios from "axios";
import { v4 as uuidv4 } from "uuid";
import { log } from "./logger";

export class AbortError extends Error {
  constructor(message = "Aborted") {
    super(message);
    this.name = "AbortError";
  }
}

// Simple async wait-until utility
async function waitFor(
  predicate: () => boolean,
  options?: { interval?: number; timeout?: number; signal?: AbortSignal },
): Promise<boolean> {
  const interval = options?.interval ?? 100;
  const timeout = options?.timeout ?? 20000;
  let waited = 0;
  while (!predicate()) {
    if (options?.signal?.aborted) return Promise.reject(new AbortError());
    if (waited >= timeout) return false;
    await new Promise((r) => setTimeout(r, interval));
    waited += interval;
  }
  return true;
}

// Dynamic EventSource import for cross-platform compatibility
let EventSource: any;

// Initialize EventSource based on environment
function initializeEventSource() {
  if (typeof window !== "undefined" && window.EventSource) {
    // Browser environment - use native EventSource
    EventSource = window.EventSource;
  } else {
    // Node.js environment - use the eventsource package
    try {
      // Use require for Node.js environments
      EventSource = require("eventsource");
    } catch (_error) {
      log.error("eventsource package not available, EventSource functionality may not work in Node.js environment");
      EventSource = null;
    }
  }
}

// Initialize immediately
initializeEventSource();

export interface INotaryInformation {
  version: string;
  publicKey: string;
  gitCommitHash: string;
  gitCommitTimestamp: string;
}

export interface VerityResponse<T> extends AxiosResponse<T, any> {
  proof?: string;
  notary_pub_key?: string;
}

export interface VerityRequestOptions {
  throwOnError?: boolean;
  proofTimeout?: number; // ms, overrides default SSE timeout
}

class VerityRequest<T> {
  private config: AxiosRequestConfig;
  private promise: Promise<AxiosResponse<T>>;
  private axiosInstance: AxiosInstance;
  private redacted: string | null = null;
  public requestId: string;
  public throwOnError = false;
  private url: string;
  private sse_is_ready = false;
  private proof: Promise<string>;
  private sse_error: unknown | null = null;

  constructor(
    axiosInstance: AxiosInstance,
    method: string,
    Url: string,
    config?: AxiosRequestConfig,
    options?: VerityRequestOptions,
    data?: any,
  ) {
    const { url: _ignoredUrl, ...rest } = config ?? {};
    this.config = rest || {};
    this.requestId = uuidv4().toString();
    this.url = Url;
    this.throwOnError = options?.throwOnError || false;

    this.axiosInstance = axiosInstance;
    this.proof = this.subscribeToProof(options?.proofTimeout || 30000);

    const instance = axios.create();

    instance.interceptors.response.use(
      async (
        response: AxiosResponse<T> & {
          proof?: string;
          notary_pub_key?: string;
        },
      ) => {
        const data = await this.proof;
        if (this.sse_error) {
          if (this.throwOnError) {
            return Promise.reject(this.sse_error);
          }
          // swallow error, leave proof undefined or empty
        }
        const index = data.indexOf("|");
        if (index >= 0) {
          response.notary_pub_key = data.slice(0, index);
          response.proof = data.slice(index + 1);
        }
        return response;
      },
    );

    instance.interceptors.request.use(async (config) => {
      const maxWaitTime = 100; // 0.1 seconds
      const interval = 20; // 20 ms
      await waitFor(() => this.sse_is_ready, { interval, timeout: maxWaitTime });
      if (!config.headers) {
        config.headers = {} as AxiosRequestHeaders;
      }
      config.headers["T-REQUEST-ID"] = this.requestId;
      config.headers["T-PROXY-URL"] = this.url;
      if (this.redacted) {
        config.headers["T-REDACTED"] = this.redacted;
      }
      return config;
    });

    this.promise = instance.request<T>({
      method,
      url: `${axiosInstance.defaults.baseURL}/proxy`,
      data,
      ...this.config,
    });
  }

  redact(value: string) {
    this.redacted = value;
    return this;
  }

  then<TResult1 = VerityResponse<T>, TResult2 = never>(
    onfulfilled?: (value: VerityResponse<T>) => TResult1 | PromiseLike<TResult1>,
    onrejected?: (reason: T) => TResult2 | PromiseLike<TResult2>,
  ) {
    return this.promise.then(onfulfilled, onrejected);
  }

  catch<TResult = never>(onrejected?: (reason: T) => TResult | PromiseLike<TResult>) {
    return this.promise.catch(onrejected);
  }

  finally(onfinally?: () => void) {
    return this.promise.finally(onfinally);
  }

  private async subscribeToProof(timeoutMs: number): Promise<string> {
    // Ensure EventSource is initialized
    if (!EventSource) {
      initializeEventSource();
    }

    const url = `${this.axiosInstance.defaults.baseURL}/proof/${this.requestId}`;
    return new Promise((resolve) => {
      if (!EventSource) {
        const err = new Error(
          "EventSource not available in this environment. Please install 'eventsource' package for Node.js environments.",
        );
        this.sse_error = err;
        log.error(`Proof SSE failed for ${this.requestId}:`, err);
        resolve("");
        return;
      }
      const es = new EventSource(url);

      const timeout = setTimeout(() => {
        es.close();
        const err = new Error("Timeout: No SSE event received");
        this.sse_error = err;
        log.error(`Proof SSE failed for ${this.requestId}:`, err);
        resolve("");
      }, timeoutMs);

      es.onopen = async (_e: any) => {
        this.sse_is_ready = true;
      };

      es.onmessage = (event: any) => {
        clearTimeout(timeout);
        es.close();
        resolve(event.data);
      };

      es.onerror = (err: any) => {
        clearTimeout(timeout);
        log.error("SSE error:", err);
        es.close();
        this.sse_error = err;
        resolve("");
      };
    });
  }
}

export class VerityClient {
  protected axios: AxiosInstance;
  private requestOptions?: VerityRequestOptions;

  constructor(
    verify_config: { prover_url: string; requestOptions?: VerityRequestOptions },
    config?: AxiosRequestConfig,
  ) {
    this.axios = axios.create({
      ...config,
      baseURL: verify_config.prover_url,
    });
    this.requestOptions = verify_config.requestOptions;
  }

  get<T>(url: string, config?: AxiosRequestConfig, options?: VerityRequestOptions) {
    const effectiveOptions = { ...(this.requestOptions || {}), ...(options || {}) } as VerityRequestOptions;
    return new VerityRequest<T>(this.axios, "get", url, config, effectiveOptions);
  }

  post<T>(url: string, config?: AxiosRequestConfig, data?: any, options?: VerityRequestOptions) {
    const effectiveOptions = { ...(this.requestOptions || {}), ...(options || {}) } as VerityRequestOptions;
    return new VerityRequest<T>(this.axios, "post", url, config, effectiveOptions, data);
  }
  /// Get the information of the connected notary
  async get_notary_info() {
    const response = await this.axios.get<INotaryInformation>("/notaryinfo");
    return response.data;
  }
}
