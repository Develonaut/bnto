import { defineConfig, devices } from "@playwright/test";
import path from "path";

/**
 * Playwright config for full-stack integration E2E tests.
 *
 * These tests run against the complete dev stack (Next.js + Convex + Go API +
 * Cloudflare tunnel) started via `task dev:all`. They exercise the real
 * execution pipeline: upload → R2 → Go engine → R2 → download.
 *
 * Run via: task e2e:integration
 * Requires: task dev:all running (or started automatically via webServer)
 */

const devPort = 4000;

export default defineConfig({
  globalTeardown: "./e2e/global-teardown.ts",
  testDir: "./e2e",
  testMatch: "*.integration.spec.ts",
  snapshotPathTemplate: path.join(
    "e2e",
    "__screenshots__",
    "{testFileName}",
    "{arg}{ext}",
  ),
  fullyParallel: false,
  timeout: 120_000,
  expect: {
    timeout: 60_000,
  },
  retries: 0,
  workers: 1,
  reporter: "html",
  use: {
    baseURL: `http://localhost:${devPort}`,
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
  webServer: {
    command: `cd ../.. && task dev:all`,
    url: `http://localhost:${devPort}`,
    reuseExistingServer: true,
    timeout: 120_000,
  },
});
