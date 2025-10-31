import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig, normalizePath } from "vite";
import { viteStaticCopy } from "vite-plugin-static-copy";
import path from "node:path";

export default defineConfig({
  // Order matters!
  plugins: [
    viteStaticCopy({
      targets: [
        {
          src: normalizePath(path.resolve(
            __dirname,
            "../../backend/bindings/output/volunteer-hub_bg.wasm",
          )),
          dest: "chunks",
        },
      ],
    }),
    sveltekit(),
  ],
  resolve: {
    alias: {
      "@volunteer-hub": path.resolve(
        __dirname,
        "../../backend/bindings/output/volunteer-hub.js",
      ),
    },
  },
  server: {
    fs: {
      allow: [
        "../../backend/bindings/output/volunteer-hub_bg.wasm",
      ],
    },
  },
});
