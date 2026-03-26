/**
 * Editor execution helpers — run recipes and export definitions.
 *
 * Extracted from editor.ts to keep files under the 250-line cap.
 */

import type { Page } from "@playwright/test";
import { expect } from "../fixtures";

// ---------------------------------------------------------------------------
// Execution
// ---------------------------------------------------------------------------

/**
 * Upload files via the editor's hidden file input and wait for execution
 * to complete.
 *
 * The editor's RunButton auto-runs on file selection — no separate
 * "upload then run" step.
 */
export async function runEditorWithFiles(
  page: Page,
  filePaths: string[],
  options?: { timeout?: number },
) {
  const { timeout = 30_000 } = options ?? {};
  const fileInput = page.getByTestId("run-file-input");
  await fileInput.setInputFiles(filePaths);

  // Wait for execution to complete
  const runButton = page.getByTestId("run-button");
  await expect(runButton).toHaveAttribute("data-phase", "completed", {
    timeout,
  });
}

/**
 * Ensure the run panel is open (idempotent).
 *
 * The run panel auto-opens when execution starts. Clicking the toolbar
 * toggle button again would CLOSE it. Check if the panel content is
 * already visible before clicking.
 */
export async function openRunPanel(page: Page) {
  const panel = page.getByTestId("panel-run");
  const alreadyOpen = await panel.isVisible().catch(() => false);
  if (!alreadyOpen) {
    await page.getByTestId("toolbar-run").click();
    await expect(panel).toBeVisible({ timeout: 3_000 });
  }
}

/** Count the number of result files in the run panel. */
export async function getResultCount(page: Page) {
  return page.getByTestId("output-file").count();
}

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

/** Export the recipe via Settings panel > Export, returns the downloaded JSON buffer. */
export async function exportRecipe(page: Page) {
  // Open settings panel — has export action regardless of node selection
  const panel = page.getByTestId("panel-settings");
  const panelVisible = await panel.isVisible().catch(() => false);
  if (!panelVisible) {
    await page.getByTestId("toolbar-settings").click();
    await expect(panel).toBeVisible({ timeout: 3_000 });
  }

  // Click Export action button
  const downloadPromise = page.waitForEvent("download");
  await page.getByTestId("panel-export").click();
  const download = await downloadPromise;

  const downloadPath = await download.path();
  expect(downloadPath).toBeTruthy();

  const fs = await import("fs");
  const buffer = fs.readFileSync(downloadPath!);
  return { buffer, filename: download.suggestedFilename() };
}
