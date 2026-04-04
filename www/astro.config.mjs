import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
	site: "https://lsb.run",
	vite: {
		plugins: [tailwindcss()],
	},
	output: "static",
});
