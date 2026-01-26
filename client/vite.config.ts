import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import { resolve } from "path";

export default defineConfig({
  plugins: [solidPlugin()],
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "pages/index.html"),
      },
    },
  },
});
