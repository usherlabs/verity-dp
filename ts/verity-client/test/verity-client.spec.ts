import axios, { Axios } from "axios";
import { VerityClient } from "../src"; // Update path accordingly

const client = new VerityClient({
	prover_url: "http://localhost:8080",
});

describe("VerityClient", () => {
	test("should get Notary", async () => {
		const notary_info = await client.get_notary_info();

		expect(notary_info.gitCommitHash.length).toBe(40);
		expect(notary_info.publicKey.startsWith("-----BEGIN PUBLIC KEY")).toBe(
			true,
		);
	});

	test("should send GET request to correct proxy URL", async () => {
		const response = await client
			.get("https://jsonplaceholder.typicode.com/posts", {
				headers: {
					"X-TEST": "SUPER_HUMAN",
				},
			})
			.redact("res:body:userId");

		expect(
			response.notary_pub_key?.startsWith("-----BEGIN PUBLIC KEY---"),
		).toBe(true);
		expect(response.status).toBe(200);
		expect(response.headers.has("t-proof-id")).toBe(true);
	});

	test("should send POST request to correct proxy URL", async () => {
		const response = await client
			.post("https://jsonplaceholder.typicode.com/posts", {
				data: { title: "foo", body: "bar", userId: 1 },

				headers: {
					Authorization:
						"Bearer xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxUAsHTzEA",
					"X-TEST-1": "SUPER_HUMAN",
				},
			})
			.redact("req:header:authorization");

		expect(
			response.notary_pub_key?.startsWith("-----BEGIN PUBLIC KEY---"),
		).toBe(true);
		expect(response.status).toBe(201);
		expect(response.headers.has("t-proof-id")).toBe(true);
	});
});
