/**
 * Editor output assertion helpers — download and verify execution output.
 *
 * Provides composable assertions for verifying magic bytes, filenames,
 * and file content from editor execution results.
 */

import fs from "fs";
import type { Page } from "@playwright/test";
import { expect } from "../fixtures";
import { MAGIC } from "../helpers";

// ---------------------------------------------------------------------------
// Download helper
// ---------------------------------------------------------------------------

/**
 * Download the first output file from the run panel.
 *
 * Returns the download object and raw file buffer for further assertions.
 * Requires the run panel to be open with at least one output file.
 */
export async function downloadFirstOutput(page: Page) {
  const outputFile = page.locator('[data-testid="output-file"]');
  await expect(outputFile).toHaveCount(1);

  const downloadPromise = page.waitForEvent("download");
  await outputFile.locator('[data-testid="download-button"]').click();
  const download = await downloadPromise;

  const downloadPath = await download.path();
  expect(downloadPath).toBeTruthy();
  const buffer = fs.readFileSync(downloadPath!);

  return { download, buffer };
}

// ---------------------------------------------------------------------------
// Magic byte assertions
// ---------------------------------------------------------------------------

/** Assert the buffer contains WebP magic bytes (RIFF header + WEBP tag). */
export function assertWebP(buffer: Buffer) {
  for (let i = 0; i < MAGIC.WEBP_RIFF.length; i++) {
    expect(buffer[i]).toBe(MAGIC.WEBP_RIFF[i]);
  }
  for (let i = 0; i < MAGIC.WEBP_TAG.length; i++) {
    expect(buffer[8 + i]).toBe(MAGIC.WEBP_TAG[i]);
  }
}

/** Assert the buffer contains JPEG magic bytes (SOI marker). */
export function assertJpeg(buffer: Buffer) {
  for (let i = 0; i < MAGIC.JPEG.length; i++) {
    expect(buffer[i]).toBe(MAGIC.JPEG[i]);
  }
}
