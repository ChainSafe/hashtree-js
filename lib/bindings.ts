import {resolve} from "node:path";
import {readFileSync} from "node:fs";
import {execSync} from "node:child_process";

function isMusl(): boolean {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== "function") {
    try {
      const lddPath = execSync("which ldd").toString().trim();
      return readFileSync(lddPath, "utf8").includes("musl");
    } catch (e) {
      return true;
    }
  } else {
    const {glibcVersionRuntime} = JSON.parse(process.report.getReport()).header;
    return !glibcVersionRuntime;
  }
}

function getBinaryName(): string {
  const {arch, platform} = process;
  switch (platform) {
    // case 'android':
    //   switch (arch) {
    //     case 'arm64':
    //       return 'hashtree.android-arm64.node';
    //     case 'arm':
    //       return 'hashtree.android-arm-eabi.node';
    //   }
    case "win32":
      switch (arch) {
        case "x64":
          return "hashtree.win32-x64-msvc.node";
        // case 'ia32':
        //   return 'hashtree.win32-ia32-msvc.node';
        // case 'arm64':
        //   return 'hashtree.win32-arm64-msvc.node';
      }
      break;
    case "darwin":
      switch (arch) {
        case "arm64":
          return "hashtree.darwin-arm64.node";
        // case 'x64':
        //   return 'hashtree.darwin-x64.node';
      }
      break;
    case "freebsd":
      // if (arch === 'x64') {
      //   return 'hashtree.freebsd-x64.node';
      // }
      break;
    case "linux":
      switch (arch) {
        case "x64":
          if (isMusl()) {
            // return 'hashtree.linux-x64-musl.node';
          } else {
            return "hashtree.linux-x64-gnu.node";
          }
          break;
        case "arm64":
          if (isMusl()) {
            // return 'hashtree.linux-arm64-musl.node';
          } else {
            return "hashtree.linux-arm64-gnu.node";
          }
          break;
        case "arm":
          if (isMusl()) {
            // return 'hashtree.linux-arm-musleabihf.node';
          } else {
            // return 'hashtree.linux-arm-gnueabihf.node';
          }
          break;
        case "riscv64":
          if (isMusl()) {
            // return 'hashtree.linux-riscv64-musl.node';
          } else {
            // return 'hashtree.linux-riscv64-gnu.node';
          }
          break;
        case "s390x":
        // return 'hashtree.linux-s390x-gnu.node';
      }
  }

  throw new Error(`HASHTREE_JS_ERROR: unsupported OS: ${platform}, architecture: ${arch}`);
}

export function getBinaryPath(rootDir: string): string {
  return resolve(rootDir, "prebuild", getBinaryName());
}
