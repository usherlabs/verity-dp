import { readFileSync } from "fs";
import { asset } from "./utils/asset";

const RECEIPT_32B = readFileSync(asset("receipt/receipt_32b.bin"));
const RECEIPT_1KB = readFileSync(asset("receipt/receipt_1kb.bin"));

export default {
  RECEIPT_32B,
  RECEIPT_1KB,
};
