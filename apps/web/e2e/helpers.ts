import path from "path";
import fs from "fs";
import type { Page } from "@playwright/test";
import { expect } from "./fixtures";

// Re-export verification helpers from focused files
export {
  getJpegDimensions,
  getPngDimensions,
  getWebPDimensions,
  getImageDimensions,
} from "./helpers/imageDimensions";
export { parseCsvOutput } from "./helpers/parseCsv";
export { assertExifStripped, assertContentPreserved } from "./helpers/contentAssertions";

// ---------------------------------------------------------------------------
// Fixture directories
// ---------------------------------------------------------------------------

export const IMAGE_FIXTURES_DIR = path.resolve(__dirname, "../../../test-fixtures/images");

export const CSV_FIXTURES_DIR = path.resolve(__dirname, "../../../test-fixtures/csv");

export const VECTOR_FIXTURES_DIR = path.resolve(__dirname, "../../../test-fixtures/vector");

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
export async function navigateToRecipe(page: Page, slug: string, _h1?: string) {
  await page.goto(`/${slug}`);
  await expect(page.getByTestId("recipe-heading")).toBeVisible();
}

/**
 * Upload files via the file input, wait for the run button to be visible.
 * Returns the run button locator.
 */
export async function uploadFiles(page: Page, filePaths: string[]) {
  const fileInput = page.getByTestId("file-input");
  await fileInput.setInputFiles(filePaths);

  const runButton = page.getByTestId("run-button");
  await expect(runButton).toBeVisible();
  await expect(runButton).toBeEnabled();

  return runButton;
}

/**
 * Click the Run button and wait for the execution to reach a terminal step.
 * Returns the run button locator.
 */
export async function runAndComplete(
  page: Page,
  options?: { timeout?: number; expectStep?: string },
) {
  const { timeout = 30_000, expectStep = "completed" } = options ?? {};

  const runButton = page.getByTestId("run-button", ":visible");
  await runButton.click();

  await expect(runButton).toHaveAttribute("data-step", expectStep, {
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
  const { outputIndex = 0, filenamePattern, magicBytes, maxSize } = options ?? {};

  const outputFile = page.getByTestId("output-file").nth(outputIndex);
  await expect(outputFile).toBeVisible();

  const downloadPromise = page.waitForEvent("download");
  await outputFile.getByTestId("download-button").click();
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
 * Run the recipe and capture the auto-download that fires on completion.
 * Returns the download and its file buffer. Verifies ZIP magic bytes for
 * batch results (multi-file runs auto-download as ZIP).
 */
export async function runAndCaptureAutoDownload(
  page: Page,
  options?: { timeout?: number; expectZip?: boolean },
) {
  const { timeout = 30_000, expectZip = true } = options ?? {};

  const downloadPromise = page.waitForEvent("download", { timeout });
  await runAndComplete(page, { timeout });
  const download = await downloadPromise;

  const downloadPath = await download.path();
  expect(downloadPath).toBeTruthy();

  const buffer = fs.readFileSync(downloadPath!);
  expect(buffer.length).toBeGreaterThan(0);

  if (expectZip) {
    for (let i = 0; i < MAGIC.ZIP.length; i++) {
      expect(buffer[i]).toBe(MAGIC.ZIP[i]);
    }
  }

  return { buffer, download };
}

/**
 * Open the config dialog by clicking the config (sliders) button.
 * Waits for the dialog content to be visible before returning.
 */
export async function openConfigDialog(page: Page) {
  await page.getByTestId("config-button").click();
  await expect(page.getByRole("dialog")).toBeVisible();
}

/**
 * Close the config dialog by pressing Escape.
 */
export async function closeConfigDialog(page: Page) {
  await page.keyboard.press("Escape");
  await expect(page.getByRole("dialog")).not.toBeVisible();
}

/**
 * Verify the buffer is a valid SVG: starts with `<svg` (possibly after whitespace).
 * Returns the SVG text for further assertions.
 */
export function assertValidSvg(buffer: Buffer): string {
  const text = buffer.toString("utf-8").trim();
  expect(text).toMatch(/^<svg[\s>]/);
  expect(text).toContain("</svg>");
  return text;
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
