import { defineConfig, devices } from "@playwright/test";

/**
 * Playwright E2E test configuration.
 *
 * Supports two modes:
 *
 * 1. Local dev server (default):
 *    - Default: port 4000 (your local `task dev`)
 *    - Agents: set E2E_PORT=4001 to avoid colliding with a running dev server
 *    - Playwright starts its own Next.js instance if no server is running
 *
 * 2. Vercel preview (CI):
 *    - Set BASE_URL to the full Vercel preview URL
 *    - Set VERCEL_AUTOMATION_BYPASS_SECRET to bypass deployment protection
 *    - No local server needed — tests run against the deployed preview
 */

const port = Number(process.env.E2E_PORT ?? 4000);
const isolated = port !== 4000;
const baseURL = process.env.BASE_URL || `http://localhost:${port}`;
const isRemote = !!process.env.BASE_URL;

export default defineConfig({
  testDir: "./e2e",
  snapshotPathTemplate:
    "{testDir}/{testFileDir}/__screenshots__/{arg}{ext}",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",
  expect: {
    toHaveScreenshot: {
      // macOS (CoreText) and Linux (FreeType) render fonts slightly differently.
      // With --font-render-hinting=none and viewport-only screenshots (no fullPage),
      // cross-platform diffs are typically <0.2%. A 2% threshold gives comfortable
      // margin while still catching real visual regressions.
      maxDiffPixelRatio: 0.02,
    },
  },
  use: {
    baseURL,
    trace: "on-first-retry",
    // Bypass Vercel deployment protection for automated testing.
    // The secret is set in GitHub Actions secrets and passed via env var.
    ...(process.env.VERCEL_AUTOMATION_BYPASS_SECRET
      ? {
          extraHTTPHeaders: {
            "x-vercel-protection-bypass":
              process.env.VERCEL_AUTOMATION_BYPASS_SECRET,
            "x-vercel-set-bypass-cookie": "true",
          },
        }
      : {}),
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: {
          args: [
            // Normalize font rendering across macOS and Linux.
            // Disables hinting and LCD subpixel antialiasing so CoreText
            // and FreeType produce near-identical output.
            "--font-render-hinting=none",
            "--disable-lcd-text",
            "--force-color-profile=srgb",
          ],
        },
      },
    },
  ],
  // Skip webServer when testing against a remote URL (Vercel preview)
  ...(isRemote
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
