import path from "node:path";
import reactRefresh from "@vitejs/plugin-react-refresh";
import { defineConfig } from "vite";
import dfxJson from "./dfx.json";

// List of all aliases for canisters
const aliases = Object.entries(dfxJson.canisters).reduce((acc, [name, _value]) => {
  // Get the network name, or `local` by default.
  const networkName = process.env["DFX_NETWORK"] || "local";
  const outputRoot = path.join(__dirname, ".dfx", networkName, "canisters", name);

  return {
    // biome-ignore lint/performance/noAccumulatingSpread: <explanation>
    ...acc,
    // biome-ignore lint/style/useTemplate: <explanation>
    ["dfx-generated/" + name]: path.join(outputRoot),
  };
}, {});

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [reactRefresh()],
  test: {
    testTimeout: 300000,
  },
  resolve: {
    alias: {
      ...aliases,
    },
  },
});
