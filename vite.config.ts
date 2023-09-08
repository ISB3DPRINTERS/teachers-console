import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

export default defineConfig({
    plugins: [react()],

    server: {
        port: 4000,

        hmr: {
            port: 4000,
            clientPort: 443,
            protocol: "wss",
        },
    },
});
