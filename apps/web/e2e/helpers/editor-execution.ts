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
  const fileInput = page.locator('[data-testid="run-file-input"]');
  await fileInput.setInputFiles(filePaths);

  // Wait for execution to complete
  const runButton = page.locator('[data-testid="run-button"]');
  await expect(runButton).toHaveAttribute("data-phase", "completed", {
    timeout,
  });
}

/**
 * Ensure the run panel is open (idempotent).
 *
 * The run panel auto-opens when execution starts. Clicking the toolbar
 * toggle button again would CLOSE it. Check if the panel is already open
 * (look for the "Logs" tab which only exists inside the run panel content)
 * before clicking.
 */
export async function openRunPanel(page: Page) {
  const logsTab = page.getByRole("tab", { name: /Logs/i });
  const alreadyOpen = await logsTab.isVisible().catch(() => false);
  if (!alreadyOpen) {
    await page.getByRole("button", { name: /run panel/i }).click();
    await expect(logsTab).toBeVisible({ timeout: 3_000 });
  }
}

/** Count the number of result files in the run panel. */
export async function getResultCount(page: Page) {
  const resultRows = page.getByRole("button", { name: /^download /i });
  return resultRows.count();
}

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

/** Export the recipe via File > Export, returns the downloaded JSON buffer. */
export async function exportRecipe(page: Page) {
  // Open file menu
  await page
    .locator('[data-testid="editor-toolbar"]')
    .getByRole("button", { name: /file menu/i })
    .click();

  // Click Export — MenuItem renders as <button>, not <menuitem>.
  // Scope to the file menu dialog to avoid matching other Export buttons.
  const downloadPromise = page.waitForEvent("download");
  const fileMenuDialog = page.getByRole("dialog").filter({ hasText: "Export" });
  await fileMenuDialog.getByRole("button", { name: /^Export$/i }).click();
  const download = await downloadPromise;

  const downloadPath = await download.path();
  expect(downloadPath).toBeTruthy();

  const fs = await import("fs");
  const buffer = fs.readFileSync(downloadPath!);
  return { buffer, filename: download.suggestedFilename() };
}
