import path from "path";
import type { Page } from "@playwright/test";
import { expect } from "./fixtures";

// Engine test fixtures — shared with Go golden tests
export const ENGINE_FIXTURES = path.resolve(
  __dirname,
  "../../../engine/tests/fixtures",
);
export const ENGINE_IMAGES = path.join(ENGINE_FIXTURES, "images");

/**
 * Wait for the anonymous session to be established.
 *
 * The BntoPageShell exposes `data-session="pending"|"ready"` on the shell
 * element. We must wait for "ready" before interacting — otherwise
 * Convex mutations will reject with "Not authenticated".
 */
export async function waitForSession(page: Page) {
  const shell = page.locator('[data-testid="bnto-shell"]');
  await expect(shell).toHaveAttribute("data-session", "ready", {
    timeout: 15_000,
  });
}

/**
 * Upload files, click Run, and wait for the pipeline to reach a terminal
 * phase (completed or failed). Returns the final phase string.
 */
export async function runPipeline(
  page: Page,
  options: { files: string[]; debugLabel: string },
): Promise<string> {
  const fileInput = page.locator('input[type="file"]');
  await fileInput.setInputFiles(options.files);

  const count = options.files.length;
  await expect(
    page.getByText(`${count} file${count === 1 ? "" : "s"} selected`),
  ).toBeVisible();

  const runButton = page.locator('[data-testid="run-button"]');
  await expect(runButton).toBeEnabled();
  await runButton.click();

  // Wait for terminal phase — generous timeout for full cloud pipeline
  await expect(runButton).toHaveAttribute(
    "data-phase",
    /(completed|failed)/,
    { timeout: 90_000 },
  );

  const phase = (await runButton.getAttribute("data-phase")) ?? "unknown";

  if (phase === "failed") {
    await page.evaluate(() => window.scrollTo(0, 0));
    await page.screenshot({
      path: `test-results/debug-${options.debugLabel}.png`,
    });
  }

  return phase;
}

/**
 * Assert the pipeline completed, results panel is visible, and take a
 * named screenshot.
 */
export async function assertCompletedWithScreenshot(
  page: Page,
  phase: string,
  screenshotName: string,
) {
  expect(phase).toBe("completed");

  const results = page.locator('[data-testid="execution-results"]');
  await expect(results).toBeVisible({ timeout: 10_000 });

  await page.evaluate(() => window.scrollTo(0, 0));
  await expect(page).toHaveScreenshot(screenshotName);
}
