import {defineConfig} from "tsup";

export default defineConfig({
  entry: ["lib/index.ts", "lib/bindings.ts"],
  dts: true,
  outDir: "dist",
  cjsInterop: true,
  splitting: false,
  sourcemap: true,
  bundle: false,
  clean: true,
  shims: true,
  format: ["cjs", "esm"],
  outExtension({format}) {
    if (format === "cjs") {
      return {
        js: ".js",
        dts: ".d.ts",
      };
    }
    return {
      js: ".mjs",
      dts: ".d.mts",
    };
  },
});
