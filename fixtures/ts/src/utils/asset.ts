import path from "path";

export function asset(asset: String) {
  const ASSETS_DIR = "../../../assets";

  return path.resolve(__dirname, `${ASSETS_DIR}/${asset}`);
}
