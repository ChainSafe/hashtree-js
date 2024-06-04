/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable @typescript-eslint/no-require-imports */
import path from "node:path";
import fs from "node:fs";

const NPM_FOLDER = path.resolve(__dirname, "..", "npm");
const {version} = require(path.resolve(__dirname, "..", "package.json"));
console.log(`Updating npm packages to version ${version}`);

(function getFilepathsRecursively(): void {
  if (!fs.existsSync(NPM_FOLDER)) {
    throw new Error("No npm folder found");
  }
  for (const subDir of fs.readdirSync(NPM_FOLDER)) {
    const subDirPath = path.resolve(NPM_FOLDER, subDir);
    const stat = fs.statSync(subDirPath);
    if (!stat.isDirectory()) {
      continue;
    }
    const pkgJsonPath = path.resolve(subDirPath, "package.json");
    const pkgJson = require(pkgJsonPath);
    pkgJson.version = version;
    fs.writeFileSync(pkgJsonPath, JSON.stringify(pkgJson, null, 2));
  }
})();
