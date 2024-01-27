import path from "path";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [],
  resolve: {
    alias: {
      "@idealjs/sapling": path.resolve(__dirname, "../../packages/core/src"),
      "@idealjs/sapling-reactive": path.resolve(
        __dirname,
        "../../packages/reactive/src",
      ),
    },
  },
});
