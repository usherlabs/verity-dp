import { readFileSync } from "fs";
import { asset } from "./utils/asset";

const IMAGE_ID = JSON.parse(readFileSync(asset("zkvm/image_id.json"), "utf8"));

export default {
  IMAGE_ID,
};
