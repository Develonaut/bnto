import path from "path";
import fs from "fs";
import type { Page } from "@playwright/test";
import { expect } from "./fixtures";

// ---------------------------------------------------------------------------
// Fixture directories
// ---------------------------------------------------------------------------

export const IMAGE_FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../test-fixtures/images",
);

export const CSV_FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../test-fixtures/csv",
);

// ---------------------------------------------------------------------------
// Magic byte constants
// ---------------------------------------------------------------------------

export const MAGIC = {
  JPEG: [0xff, 0xd8, 0xff] as const,
  PNG: [0x89, 0x50, 0x4e, 0x47] as const,
  WEBP_RIFF: [0x52, 0x49, 0x46, 0x46] as const, // bytes 0-3
  WEBP_TAG: [0x57, 0x45, 0x42, 0x50] as const, // bytes 8-11
  ZIP: [0x50, 0x4b, 0x03, 0x04] as const,
} as const;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/**
 * Navigate to a recipe page and wait for the heading to be visible.
 */
export async function navigateToRecipe(page: Page, slug: string, h1: string) {
  await page.goto(`/${slug}`);
  await expect(
    page.getByRole("heading", { name: h1 }),
  ).toBeVisible();
}

/**
 * Assert that the bnto-shell has data-execution-mode="browser".
 */
export async function assertBrowserExecution(page: Page) {
  const shell = page.locator('[data-testid="bnto-shell"]');
  await expect(shell).toHaveAttribute("data-execution-mode", "browser");
}

/**
 * Upload files via the file input, wait for the file count text and run button.
 * Returns the run button locator.
 */
export async function uploadFiles(page: Page, filePaths: string[]) {
  const fileInput = page.locator('input[type="file"]');
  await fileInput.setInputFiles(filePaths);

  const count = filePaths.length;
  await expect(
    page.getByText(`${count} file${count === 1 ? "" : "s"} selected`),
  ).toBeVisible();

  const runButton = page.locator('[data-testid="run-button"]:visible');
  await expect(runButton).toBeEnabled();

  return runButton;
}

/**
 * Click the Run button and wait for the execution to reach a terminal phase.
 * Returns the run button locator.
 */
export async function runAndComplete(
  page: Page,
  options?: { timeout?: number; expectPhase?: string },
) {
  const { timeout = 30_000, expectPhase = "completed" } = options ?? {};

  const runButton = page.locator('[data-testid="run-button"]:visible');
  await runButton.click();

  await expect(runButton).toHaveAttribute("data-phase", expectPhase, {
    timeout,
  });

  return runButton;
}

/**
 * Download the output file at the given index (default: first), verify magic
 * bytes and optional size constraints. Returns the downloaded file buffer.
 */
export async function downloadAndVerify(
  page: Page,
  options?: {
    outputIndex?: number;
    filenamePattern?: RegExp;
    magicBytes?: readonly number[];
    maxSize?: number;
  },
) {
  const { outputIndex = 0, filenamePattern, magicBytes, maxSize } =
    options ?? {};

  const outputFile = page.locator('[data-testid="output-file"]').nth(outputIndex);
  await expect(outputFile).toBeVisible();

  const downloadPromise = page.waitForEvent("download");
  await outputFile.getByRole("button", { name: /download/i }).click();
  const download = await downloadPromise;

  if (filenamePattern) {
    expect(download.suggestedFilename()).toMatch(filenamePattern);
  }

  const downloadPath = await download.path();
  expect(downloadPath).toBeTruthy();

  const buffer = fs.readFileSync(downloadPath!);
  expect(buffer.length).toBeGreaterThan(0);

  if (magicBytes) {
    for (let i = 0; i < magicBytes.length; i++) {
      expect(buffer[i]).toBe(magicBytes[i]);
    }
  }

  if (maxSize !== undefined) {
    expect(buffer.length).toBeLessThanOrEqual(maxSize);
  }

  return buffer;
}

/**
 * Click "Download All" (last matching button), verify ZIP magic bytes.
 * Returns the downloaded file buffer.
 */
export async function downloadAllAsZip(page: Page) {
  const downloadAllBtn = page
    .getByRole("button", { name: /download all/i })
    .last();
  await expect(downloadAllBtn).toBeVisible();

  const downloadPromise = page.waitForEvent("download");
  await downloadAllBtn.click();
  const download = await downloadPromise;

  const downloadPath = await download.path();
  expect(downloadPath).toBeTruthy();

  const buffer = fs.readFileSync(downloadPath!);
  expect(buffer.length).toBeGreaterThan(100);

  // Verify ZIP magic bytes
  for (let i = 0; i < MAGIC.ZIP.length; i++) {
    expect(buffer[i]).toBe(MAGIC.ZIP[i]);
  }

  return { buffer, download };
}

/**
 * Verify WebP magic bytes (RIFF header at bytes 0-3 and WEBP tag at bytes 8-11).
 */
export function assertWebPBytes(buffer: Buffer) {
  for (let i = 0; i < MAGIC.WEBP_RIFF.length; i++) {
    expect(buffer[i]).toBe(MAGIC.WEBP_RIFF[i]);
  }
  for (let i = 0; i < MAGIC.WEBP_TAG.length; i++) {
    expect(buffer[8 + i]).toBe(MAGIC.WEBP_TAG[i]);
  }
}
