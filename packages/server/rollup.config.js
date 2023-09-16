import alias from "@rollup/plugin-alias";
import esbuild from "rollup-plugin-esbuild";
import { visualizer } from "rollup-plugin-visualizer";

const config = {
  input: ["./src/index.ts"],
  plugins: [
    alias({
      entries: [
        {
          find: "@idealjs/mono-template-core",
          replacement: "../../packages/core/src/index.ts",
        },
      ],
    }),
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
};

export default config;
