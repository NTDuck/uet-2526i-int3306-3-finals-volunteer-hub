import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [sveltekit()],
	resolve: {
		alias: {
			"@volunteer-hub": path.resolve(__dirname, "../../backend/bindings/output/volunteer-hub.js"),
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
