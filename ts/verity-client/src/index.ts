import axios from "axios";
import { v4 as uuidv4 } from "uuid";
import type {
	AxiosInstance,
	AxiosRequestConfig,
	AxiosResponse,
	AxiosResponseHeaders,
	RawAxiosResponseHeaders,
} from "axios";

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
		} catch (error) {
			console.warn(
				"eventsource package not available, EventSource functionality may not work in Node.js environment",
			);
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

class VerityRequest<T> {
	private config: AxiosRequestConfig;
	private promise: Promise<AxiosResponse<T>>;
	private axiosInstance: AxiosInstance;
	private redacted: string | null = null;
	public requestId: string;
	private url: string;
	private sse_is_ready = false;
	private proof: Promise<string>;

	constructor(
		axiosInstance: AxiosInstance,
		method: string,
		Url: string,
		config?: AxiosRequestConfig,
		data?: any,
	) {
		const { url, ...rest } = config ?? {};
		this.config = rest || {};
		this.requestId = uuidv4().toString();
		this.url = Url;

		this.axiosInstance = axiosInstance;
		this.proof = this.subscribeToProof().catch((err) => {
			console.error(`Proof SSE failed for ${this.requestId}:`, err);
			// re-throw so downstream still sees the error
			throw err;
		});

		const instance = axios.create();

		instance.interceptors.response.use(
			async (
				response: AxiosResponse<T> & {
					proof?: string;
					notary_pub_key?: string;
				},
			) => {
				try {
					const data = await this.proof;
					const index = data.indexOf("|");
					response.notary_pub_key = data.slice(0, index);
					response.proof = data.slice(index + 1);
					return response;
				} catch (error) {
					console.log({ error });
					return response;
				}
			},
		);

		instance.interceptors.request.use(async (config) => {
			const maxWaitTime = 100; // 0.1 seconds
			const interval = 20; // 20 ms
			let waited = 0;

			while (!this.sse_is_ready) {
				if (waited >= maxWaitTime) {
					break;
				}
				await new Promise((resolve) => setTimeout(resolve, interval));
				waited += interval;
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
		onfulfilled?: (
			value: VerityResponse<T>,
		) => TResult1 | PromiseLike<TResult1>,
		onrejected?: (reason: T) => TResult2 | PromiseLike<TResult2>,
	) {
		return this.promise.then(onfulfilled, onrejected);
	}

	catch<TResult = never>(
		onrejected?: (reason: T) => TResult | PromiseLike<TResult>,
	) {
		return this.promise.catch(onrejected);
	}

	finally(onfinally?: () => void) {
		return this.promise.finally(onfinally);
	}

	private async subscribeToProof(timeoutMs = 1800000): Promise<string> {
		// Ensure EventSource is initialized
		if (!EventSource) {
			initializeEventSource();
		}

		const url = `${this.axiosInstance.defaults.baseURL}/proof/${this.requestId}`;
		return new Promise((resolve, reject) => {
			if (!EventSource) {
				reject(
					new Error(
						"EventSource not available in this environment. Please install 'eventsource' package for Node.js environments.",
					),
				);
				return;
			}
			const es = new EventSource(url);

			const timeout = setTimeout(() => {
				es.close();
				reject(new Error("Timeout: No SSE event received"));
			}, timeoutMs);

			es.onopen = async (e: any) => {
				this.sse_is_ready = true;
			};

			es.onmessage = (event: any) => {
				clearTimeout(timeout);
				es.close();
				resolve(event.data);
			};

			es.onerror = (err: any) => {
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

	constructor(
		verify_config: { prover_url: string },
		config?: AxiosRequestConfig,
	) {
		this.axios = axios.create({
			...config,
			baseURL: verify_config.prover_url,
		});
	}

	get<T>(url: string, config?: AxiosRequestConfig) {
		return new VerityRequest<T>(this.axios, "get", url, config);
	}

	post<T>(url: string, config?: AxiosRequestConfig, data?: any) {
		return new VerityRequest<T>(this.axios, "post", url, config, data);
	}

	/// Get the information of the connected notary
	async get_notary_info() {
		const response = await this.axios.get<INotaryInformation>("/notaryinfo");
		return response.data;
	}
}
