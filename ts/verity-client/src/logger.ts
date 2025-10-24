import { Logger } from "tslog";

const logLevel = Number.parseInt(process.env.LOG_LEVEL || "3", 10) || 3;

export const log = new Logger({
  name: "VerityTsClient",
  minLevel: logLevel,
});
