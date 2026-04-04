import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
	plugins: [wasm()],
	root: ".",
	publicDir: "assets",
	build: {
		outDir: "dist",
		rollupOptions: {
			input: {
				main: "./index.html",
			},
		},
	},
	server: {
		port: 8000,
		proxy: {
			"/public": "http://localhost:8080",
		},
	},
});
