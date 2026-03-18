import { defineConfig, devices } from "@playwright/test";

/**
 * Playwright E2E test configuration.
 *
 * Three-stage execution for stability:
 *   - "browser" project: pages, browser journeys, telemetry — fully parallel
 *   - "auth" project: auth lifecycle + behavior tests — serial (--workers=1)
 *   - "editor" project: editor component + journey tests — serial (--workers=1)
 *
 * Auth tests hit Convex for real sign-ups/sign-ins and manipulate cookies.
 * Under parallel load the dev server gets overwhelmed (ERR_CONNECTION_REFUSED)
 * and auth operations time out. Serial execution is both faster (no retries)
 * and 100% reliable.
 *
 * ReactFlow continuously re-renders, so parallel editor tests lose DOM references
 * between Playwright's element resolution and click dispatch. Serial execution
 * eliminates the CPU contention that triggers this.
 *
 * Usage:
 *   task e2e              # all three stages
 *   task e2e:browser      # parallel tests only
 *   task e2e:auth         # auth tests only (serial)
 *   task e2e:editor       # editor tests only (serial)
 */

const port = 4000;

export default defineConfig({
  globalTeardown: "./e2e/global-teardown.ts",
  testDir: "./e2e",
  snapshotPathTemplate: "{testDir}/{testFileDir}/__screenshots__/{arg}{ext}",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 1,
  workers: process.env.CI ? 1 : 4,
  reporter: "html",
  expect: {
    toHaveScreenshot: {
      maxDiffPixelRatio: 0.05,
    },
  },
  use: {
    baseURL: `http://localhost:${port}`,
    trace: "on-first-retry",
    contextOptions: { reducedMotion: "reduce" },
  },
  projects: [
    {
      name: "browser",
      testDir: "./e2e",
      testIgnore: ["**/editor/**", "**/auth/**"],
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "auth",
      testDir: "./e2e",
      testMatch: ["**/auth/**"],
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "editor",
      testDir: "./e2e",
      testMatch: ["**/editor/**"],
      use: { ...devices["Desktop Chrome"] },
    },
  ],
  webServer: {
    command: `pnpm turbo run dev`,
    url: `http://localhost:${port}`,
    reuseExistingServer: true,
    timeout: 120_000,
  },
});
