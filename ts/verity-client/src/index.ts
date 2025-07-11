import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from "axios";
import axios from "axios";
import EventSource from "eventsource";
import { v4 as uuidv4 } from "uuid";

interface INotaryInformation {
  version: string;
  publicKey: string;
  gitCommitHash: string;
  gitCommitTimestamp: string;
}

class VerityRequest<T> {
  private config: AxiosRequestConfig;
  private promise: Promise<AxiosResponse<T>>;
  private axiosInstance: AxiosInstance;
  private redacted: string | null = null;
  public proofId: string | null;
  public requestId: string;

  constructor(axiosInstance: AxiosInstance, method: string, url: string, config?: AxiosRequestConfig, data?: any) {
    this.config = config || {};
    this.proofId = null;
    this.requestId = uuidv4().toString();

    this.axiosInstance = axiosInstance;

    const instance = axios.create();

    instance.interceptors.response.use(
      async (
        response: AxiosResponse<T> & {
          proof?: string;
          notary_pub_key?: string;
        },
      ) => {
        this.proofId = `${response.headers["t-proof-id"]}`;
        const data = await this.subscribeToProof();
        const index = data.indexOf("|");
        response.notary_pub_key = data.slice(0, index);
        response.proof = data.slice(index + 1);
        return response;
      },
    );

    instance.interceptors.request.use(async (config) => {
      config.headers["T-REQUEST-ID"] = this.requestId;
      config.headers["T-PROXY-URL"] = url;
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
      transformRequest: [
        (data, headers) => {
          headers["T-REQUEST-ID"] = this.requestId;
          headers["T-PROXY-URL"] = url;
          return data;
        },
      ],
    });
  }

  redact(value: string) {
    this.redacted = value;
    return this;
  }

  then<TResult1 = AxiosResponse<T> & { proof?: string; notary_pub_key?: string }, TResult2 = never>(
    onfulfilled?: (
      value: AxiosResponse<T> & { proof?: string; notary_pub_key?: string },
    ) => TResult1 | PromiseLike<TResult1>,
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

  private async subscribeToProof(timeoutMs = 100000): Promise<string> {
    const url = `${this.axiosInstance.defaults.baseURL}/proof/${this.requestId}`;
    return new Promise((resolve, reject) => {
      const es = new EventSource(url);

      const timeout = setTimeout(() => {
        es.close();
        reject(new Error("Timeout: No SSE event received"));
      }, timeoutMs);

      es.onopen = async () => {
        await this.axiosInstance.get(`${this.axiosInstance.defaults.baseURL}/proxy`, {
          headers: {
            "T-REQUEST-ID": this.requestId,
            "T-PROXY-URL": "https://fast.com",
          },
        });
      };

      es.onmessage = (event) => {
        clearTimeout(timeout);
        es.close();
        resolve(event.data);
      };

      es.onerror = (err) => {
        clearTimeout(timeout);
        console.error("SSE error:", err);
        es.close();
        reject(err);
      };
    });
  }
}

export class VerityClient {
  protected axios: AxiosInstance;

  constructor(verify_config: { prover_url: string }) {
    this.axios = axios.create({
      baseURL: verify_config.prover_url,
    });
  }

  get<T>(url: string, config?: AxiosRequestConfig) {
    return new VerityRequest<T>(this.axios, "get", url, config);
  }

  post<T>(url: string, data?: any, config?: AxiosRequestConfig) {
    return new VerityRequest<T>(this.axios, "post", url, config, data);
  }

  /// Get the information of the connected notary
  async get_notary_info() {
    const response = await this.axios.get<INotaryInformation>("/notaryinfo");
    return response.data;
  }
}
