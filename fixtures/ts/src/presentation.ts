import { readFileSync } from "fs";
import { asset } from "./utils/asset";

const PRESENTATION_32B = readFileSync(asset("presentation/presentation_32b.json"), "utf8");
const PRESENTATION_1KB = readFileSync(asset("presentation/presentation_1kb.json"), "utf8");

export default {
  PRESENTATION_32B,
  PRESENTATION_1KB,
};
