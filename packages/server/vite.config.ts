import type { FastifyInstance } from "fastify";
import path from "path";
import { defineConfig } from "vite";

export default defineConfig({
  server: {
    port: 3100,
  },
  plugins: [
    {
      name: "fastify",
      config: () => {
        return {
          build: {
            rollupOptions: {
              input: "./src/index.ts",
            },
          },
        };
      },
      configureServer: (server) => {
        server.middlewares.use(async (req, res, next) => {
          const module = await server.ssrLoadModule("./src/app.ts");
          const app = module.default as FastifyInstance;
          await app.ready();
          app.routing(req, res);
        });
      },
    },
  ],
  resolve: {
    alias: {
      "@idealjs/mono-template-core": path.resolve(
        __dirname,
        "../../packages/core/src/index.ts"
      ),
    },
  },
});
