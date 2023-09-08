import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
    plugins: [
        react(),
        viteStaticCopy({
            targets: [
                {
                    src: "./src/server.js",
                    dest: "./",
                },
            ],
        }),
    ],

    server: {
        port: 4000,

        hmr: {
            port: 4000,
            clientPort: 443,
            protocol: "wss",
        },
    },
});
