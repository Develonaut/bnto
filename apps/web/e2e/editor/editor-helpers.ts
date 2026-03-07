import path from "path";
import type { Page } from "@playwright/test";
import { expect } from "../fixtures";

// ---------------------------------------------------------------------------
// Fixture directories (same as browser execution helpers)
// ---------------------------------------------------------------------------

export const IMAGE_FIXTURES_DIR = path.resolve(__dirname, "../../../../test-fixtures/images");

export const CSV_FIXTURES_DIR = path.resolve(__dirname, "../../../../test-fixtures/csv");

// ---------------------------------------------------------------------------
// Navigation
// ---------------------------------------------------------------------------

/**
 * Navigate to /editor (blank) or /editor?from={slug} (predefined recipe).
 * Waits for the canvas to be ready.
 */
export async function navigateToEditor(page: Page, slug?: string) {
  const url = slug ? `/editor?from=${slug}` : "/editor";
  await page.goto(url);
  await page.waitForSelector('[data-testid="recipe-editor"]', {
    timeout: 15000,
  });
}

// ---------------------------------------------------------------------------
// Node interaction
// ---------------------------------------------------------------------------

/**
 * Count the number of node cards currently visible on the canvas.
 * Excludes placeholder nodes.
 */
export async function getNodeCount(page: Page) {
  return page.locator('[data-testid="node-card"]').count();
}

/**
 * Click a node on the canvas by its label text.
 * ReactFlow wraps each custom node; we find the node-card with matching text.
 */
export async function selectNodeByLabel(page: Page, label: string) {
  const node = page.locator('[data-testid="node-card"]').filter({ hasText: label });
  await node.waitFor({ timeout: 5000 });
  await node.click({ force: true });
}

/**
 * Click the Input node on the canvas.
 */
export async function selectInputNode(page: Page) {
  await selectNodeByLabel(page, "Input");
}

/**
 * Click the Output node on the canvas.
 */
export async function selectOutputNode(page: Page) {
  await selectNodeByLabel(page, "Output");
}

// ---------------------------------------------------------------------------
// Execution
// ---------------------------------------------------------------------------

/**
 * Upload files via the editor's hidden file input (triggered by Run button).
 * Sets files directly on the input element rather than clicking Run first.
 */
export async function editorUploadFiles(page: Page, filePaths: string[]) {
  const fileInput = page.locator('[data-testid="editor-file-input"]');
  await fileInput.setInputFiles(filePaths);
}

/**
 * Click the Run button in the editor toolbar.
 * If phase is idle, this opens the file picker — use editorRunWithFiles instead.
 */
export async function clickRunButton(page: Page) {
  const runButton = page.locator('[data-testid="editor-run-button"]');
  await runButton.click();
}

/**
 * Full execution flow: set files on input, then trigger run via the toolbar.
 * Waits for the run button to reach the expected terminal phase.
 *
 * Returns the run button locator.
 */
export async function editorRunWithFiles(
  page: Page,
  filePaths: string[],
  options?: { timeout?: number; expectPhase?: string },
) {
  const { timeout = 30_000, expectPhase = "completed" } = options ?? {};

  // Set files on the hidden input
  const fileInput = page.locator('[data-testid="editor-file-input"]');
  await fileInput.setInputFiles(filePaths);

  const runButton = page.locator('[data-testid="editor-run-button"]');

  // Wait for terminal phase
  await expect(runButton).toHaveAttribute("data-phase", expectPhase, {
    timeout,
  });

  return runButton;
}

/**
 * Wait for the editor run button to reach a specific phase.
 */
export async function waitForPhase(page: Page, phase: string, timeout = 30_000) {
  const runButton = page.locator('[data-testid="editor-run-button"]');
  await expect(runButton).toHaveAttribute("data-phase", phase, { timeout });
}

/**
 * Get the current phase of the editor run button.
 */
export async function getPhase(page: Page) {
  const runButton = page.locator('[data-testid="editor-run-button"]');
  return runButton.getAttribute("data-phase");
}

/**
 * Open the config panel if it isn't already open.
 */
export async function ensureConfigPanelOpen(page: Page) {
  // The config panel toggle is in the toolbar
  const configButton = page.getByRole("button", { name: /config/i });
  // Check if panel content is visible
  const panelHeader = page.locator('[class*="PanelHeader"], [class*="panel"]').first();
  const isOpen = await panelHeader.isVisible().catch(() => false);
  if (!isOpen) {
    await configButton.click();
  }
}
