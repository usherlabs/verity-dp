import { readFileSync } from "fs";
import { asset } from "./utils/asset";
import { to_sec1_bytes } from "./utils/crypto";

const PUB_KEY = to_sec1_bytes(readFileSync(asset("notary/notary.pub"), "utf8"));

export default {
  PUB_KEY,
};
