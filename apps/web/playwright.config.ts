import { defineConfig, devices } from "@playwright/test";

/**
 * Playwright E2E test configuration.
 *
 * Three modes:
 * 1. Remote (CI against Vercel preview): set PLAYWRIGHT_BASE_URL — no local server started
 * 2. Isolated (agents): set E2E_PORT=4001 — starts own Next.js on that port
 * 3. Default: port 4000 — reuses your local `task dev`
 */

const remoteUrl = process.env.PLAYWRIGHT_BASE_URL;
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
  },
  projects: [
    {
      name: "chromium",
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
