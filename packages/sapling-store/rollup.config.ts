import { defineConfig } from "rollup";
import esbuild from "rollup-plugin-esbuild";
import { visualizer } from "rollup-plugin-visualizer";

const config = defineConfig([
  {
    input: ["./src/index.ts"],
    plugins: [
      esbuild(),
      visualizer({
        emitFile: true,
        filename: "stats.html",
      }),
    ],
    output: [
      {
        dir: "dist/umd",
        name: "saplingStore",
        format: "umd",
      },
      {
        dir: "dist/esm",
        name: "saplingStore",
        format: "esm",
      },
    ],
  },
]);

export default config;
