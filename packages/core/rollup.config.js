import { defineConfig } from "rollup";
import { dts } from "rollup-plugin-dts";
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
        dir: "dist/umd",
        name: "index",
        format: "umd",
      },
      {
        dir: "dist/esm",
        name: "index",
        format: "esm",
      },
    ],
  },
  {
    input: "./dist/jsx-runtime.d.ts",
    output: {
      file: "jsx-runtime.d.ts",
    },
    plugins: [dts()],
  },
]);

export default config;
