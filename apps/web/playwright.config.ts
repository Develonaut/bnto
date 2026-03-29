import { defineConfig, devices } from "@playwright/test";

/**
 * Playwright E2E test configuration.
 *
 * Three modes:
 * 1. Remote (CI against Vercel preview): set PLAYWRIGHT_BASE_URL — no local server started
 * 2. Isolated (agents): set E2E_PORT=4001 — starts own Next.js on that port
 * 3. Default: port 4000 — reuses your local `task dev`
 *
 * Two-stage execution for stability:
 *   - "browser" project: pages, browser journeys, telemetry — fully parallel
 *   - "auth" project: auth lifecycle + behavior tests — serial (--workers=1)
 *
 * Auth tests hit Convex for real sign-ups/sign-ins and manipulate cookies.
 * Under parallel load the dev server gets overwhelmed (ERR_CONNECTION_REFUSED)
 * and auth operations time out. Serial execution is both faster (no retries)
 * and 100% reliable.
 *
 * Usage:
 *   task e2e              # both stages
 *   task e2e:browser      # parallel tests only
 *   task e2e:auth         # auth tests only (serial)
 */

const remoteUrl = process.env.PLAYWRIGHT_BASE_URL;
const bypassSecret = process.env.VERCEL_AUTOMATION_BYPASS_SECRET;
const port = Number(process.env.E2E_PORT ?? 4000);
const isolated = port !== 4000;

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
    baseURL: remoteUrl || `http://localhost:${port}`,
    trace: "on-first-retry",
    contextOptions: { reducedMotion: "reduce" },
    // Bypass Vercel Deployment Protection when testing against preview URLs
    ...(bypassSecret ? { extraHTTPHeaders: { "x-vercel-protection-bypass": bypassSecret } } : {}),
  },
  projects: [
    {
      name: "browser",
      testDir: "./e2e",
      testIgnore: ["**/auth/**"],
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "auth",
      testDir: "./e2e",
      testMatch: ["**/auth/**"],
      use: { ...devices["Desktop Chrome"] },
    },
  ],
  // Disable webServer when testing against a remote URL (e.g. Vercel preview in CI)
  ...(remoteUrl
    ? {}
    : {
        webServer: {
          command: isolated
            ? `NEXT_DIST_DIR=.next-e2e npx next dev --port ${port}`
            : `pnpm turbo run dev`,
          url: `http://localhost:${port}`,
          reuseExistingServer: true,
          timeout: 120_000,
        },
      }),
});
