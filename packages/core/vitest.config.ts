import path from "path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
  },
  resolve: {
    alias: {
      "@idealjs/sapling-reactive": path.resolve(
        __dirname,
        "../../packages/reactive/src",
      ),
    },
  },
});
