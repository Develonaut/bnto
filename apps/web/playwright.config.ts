import { defineConfig, devices } from "@playwright/test";

/**
 * Playwright E2E test configuration.
 *
 * Tests always run against a local dev server:
 * - Default: port 4000 (your local `task dev`)
 * - Agents: set E2E_PORT=4001 to avoid colliding with a running dev server
 * - Playwright starts its own Next.js instance if no server is running
 */

const port = Number(process.env.E2E_PORT ?? 4000);
const isolated = port !== 4000;

export default defineConfig({
  testDir: "./e2e",
  snapshotPathTemplate:
    "{testDir}/{testFileDir}/__screenshots__/{arg}{ext}",
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
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
  webServer: {
    command: isolated
      ? `NEXT_DIST_DIR=.next-e2e npx next dev --port ${port}`
      : `pnpm turbo run dev`,
    url: `http://localhost:${port}`,
    reuseExistingServer: true,
    timeout: 120_000,
  },
});
