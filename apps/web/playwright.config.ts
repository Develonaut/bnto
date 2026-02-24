import { defineConfig, devices } from "@playwright/test";
import path from "path";

const port = 3100;

export default defineConfig({
  testDir: "./e2e",
  testIgnore: "*.integration.spec.ts",
  snapshotPathTemplate: path.join(
    "e2e",
    "__screenshots__",
    "{testFileName}",
    "{arg}{ext}"
  ),
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",
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
    command: `NEXT_DIST_DIR=.next-e2e npx next dev --port ${port}`,
    url: `http://localhost:${port}`,
    reuseExistingServer: !process.env.CI,
  },
});
