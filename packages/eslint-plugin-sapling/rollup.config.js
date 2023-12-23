import { defineConfig } from "rollup";
import esbuild from "rollup-plugin-esbuild";
import { visualizer } from "rollup-plugin-visualizer";

const config = defineConfig([
  {
    input: ["./src/index.ts"],
    plugins: [
      esbuild.default(),
      visualizer({
        emitFile: true,
        filename: "stats.html",
      }),
    ],
    output: [
      {
        file: "dist/cjs/index.cjs",
        format: "cjs",
      },
      {
        file: "dist/esm/index.js",
        format: "esm",
      },
    ],
  },
]);

export default config;
