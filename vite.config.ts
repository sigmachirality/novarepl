import { defineConfig, Plugin } from "vite"
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const crossOriginIsolatedPlugin: Plugin = {
  name: 'log-request-middleware',
  configureServer(server) {
      server.middlewares.use((_, res, next) => {
          res.setHeader("Access-Control-Allow-Origin", "*");
          res.setHeader("Access-Control-Allow-Methods", "GET");
          res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
          res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
          res.setHeader("Cross-Origin-Resource-Policy", "same-site");
          next();
      });
  }
};

export default defineConfig({
    resolve: {
        alias: {
            "@wasmer/wasi/lib/polyfills/buffer": "./src/worker/buffer.ts",
            "web-worker": "./src/worker/buffer.ts",
            readline: "./src/worker/buffer.ts",
            crypto: "./src/worker/crypto.ts",
            constants: "./src/worker/constants.ts",
            fs: "./src/worker/filesystem.ts",
        },
    },
    define: {
        "global.TYPED_ARRAY_SUPPORT": "true",
        "process.browser": "true",
        // "if (singleThread)": "if (true)",
    },
    base: "",
    build: {
        assetsDir: "",
        target: ["es2020"],
    },
    optimizeDeps: {
        esbuildOptions: { target: "es2020", supported: { bigint: true } },
        exclude: ["nova_scotia_browser"]
      },
    plugins: [
      wasm(),
      topLevelAwait(),
      crossOriginIsolatedPlugin
    ],
    worker: {
      plugins: [
        wasm(),
        topLevelAwait(),
      ]
    }
})
