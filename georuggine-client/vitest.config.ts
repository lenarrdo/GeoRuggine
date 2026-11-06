import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

// Dedicated configuration for the frontend test suite.
// It is kept separate from vite.config.ts so that `tauri dev/build`
// are not affected by the test-only settings.
export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: ["./src/test/setup.ts"],
    css: false,
    clearMocks: true,
    restoreMocks: true,
    include: ["src/**/*.{test,spec}.{ts,tsx}"],
    coverage: {
      provider: "v8",
      reporter: ["text", "html", "lcov"],
      include: ["src/**/*.{ts,tsx}"],
      exclude: [
        "src/main.tsx",
        "src/**/*.d.ts",
        "src/test/**",
        "src/**/*.test.{ts,tsx}",
        "src/types/**",
      ],
    },
  },
});
