import path from "path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    setupFiles: path.resolve(__dirname, "./vitest.setup.ts"),
  },
  resolve: {
    alias: {
      "@idealjs/sapling-reactive": path.resolve(__dirname, "../reactive/src"),
    },
  },
});
