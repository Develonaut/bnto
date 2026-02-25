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
    toHaveScreenshot: {
      // Integration tests have dynamic content (timing text like "Completed in Xs",
      // download URL loading states) that causes small pixel diffs between runs.
      // Allow up to 2% pixel difference to avoid flaky screenshot assertions.
      maxDiffPixelRatio: 0.02,
    },
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
