/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/explicit-function-return-type */
const {resolve} = require("path");
const {getBinaryPath} = require("./bindings");

const rootDir = resolve(__dirname, "..");
const binaryPath = getBinaryPath(rootDir);
module.exports = require(binaryPath);
