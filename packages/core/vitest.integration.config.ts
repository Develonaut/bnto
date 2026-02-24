import { defineConfig } from "vitest/config";

/**
 * Integration test config — separate from unit tests.
 *
 * These tests call real Convex dev functions via ConvexHttpClient.
 * They require `task dev:all` to be running (Convex dev + Go API + tunnel).
 *
 * Run: pnpm --filter @bnto/core test:integration
 * Or:  task core:integration
 */
export default defineConfig({
  test: {
    include: ["src/__tests__/integration/**/*.test.ts"],
    environment: "node",
    testTimeout: 60_000,
    hookTimeout: 30_000,
    // Pool of 1 — sequential execution avoids rate limiting on auth actions
    pool: "forks",
    poolOptions: {
      forks: { singleFork: true },
    },
  },
});
