import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {resolve, dirname} from "node:path";
import {getBinaryPath} from "./bindings";

const require = createRequire(import.meta.url);
const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = resolve(__dirname, "..");
const binaryPath = getBinaryPath(rootDir);
const bindings = require(binaryPath);

export default bindings;

const {hash, hashInto} = bindings;
export {hash, hashInto};
