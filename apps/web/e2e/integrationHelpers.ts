import path from "path";
import fs from "fs";
import type { Download, Page } from "@playwright/test";
import { expect } from "./fixtures";

// Engine test fixtures — shared with Go golden tests
export const ENGINE_FIXTURES = path.resolve(
  __dirname,
  "../../../engine/tests/fixtures",
);
export const ENGINE_IMAGES = path.join(ENGINE_FIXTURES, "images");

/**
 * Full-page screenshot options — all integration screenshots capture the
 * entire page so nothing is clipped by the viewport.
 */
const SCREENSHOT_OPTIONS = { fullPage: true } as const;

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
 * Wait for the current user's ID to be available on the shell element.
 *
 * Returns the userId string. Useful for verifying identity across
 * session transitions (e.g., anonymous → password upgrade).
 */
export async function waitForUserId(page: Page): Promise<string> {
  const shell = page.locator('[data-testid="bnto-shell"]');
  // Wait for data-user-id to be a non-empty string
  await expect(shell).toHaveAttribute("data-user-id", /.+/, {
    timeout: 15_000,
  });
  const userId = await shell.getAttribute("data-user-id");
  if (!userId) throw new Error("data-user-id was empty after wait");
  return userId;
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
    await page.screenshot({
      path: `test-results/debug-${options.debugLabel}.png`,
      fullPage: true,
    });
  }

  return phase;
}

/**
 * Assert the pipeline completed, results panel is visible, and take a
 * named full-page screenshot.
 */
export async function assertCompletedWithScreenshot(
  page: Page,
  phase: string,
  screenshotName: string,
) {
  expect(phase).toBe("completed");

  const results = page.locator('[data-testid="execution-results"]');
  await expect(results).toBeVisible({ timeout: 10_000 });

  await expect(page).toHaveScreenshot(screenshotName, SCREENSHOT_OPTIONS);
}

// ---------------------------------------------------------------------------
// Progress-aware helpers — make it easy to snapshot execution progress
// ---------------------------------------------------------------------------

/**
 * Wait for execution progress to reach a specific status and take a snapshot.
 *
 * The ExecutionProgress component exposes `data-status` on its root element:
 *   - "loading" — Execution data not yet loaded
 *   - "pending" — Queued, waiting to start
 *   - "running" — Actively processing nodes
 *   - "completed" — All nodes done
 *   - "failed" — Execution errored
 *
 * Per-node progress rows expose `data-node-status` and `data-node-id`.
 *
 * Usage:
 * ```ts
 * await waitForExecutionStatus(page, "running", "my-test-02-running.png");
 * ```
 */
export async function waitForExecutionStatus(
  page: Page,
  status: "loading" | "pending" | "running" | "completed" | "failed",
  screenshotName?: string,
  options?: { timeout?: number },
) {
  const progress = page.locator('[data-testid="execution-progress"]');
  await expect(progress).toHaveAttribute("data-status", status, {
    timeout: options?.timeout ?? 30_000,
  });

  if (screenshotName) {
    await expect(page).toHaveScreenshot(screenshotName, SCREENSHOT_OPTIONS);
  }
}

/**
 * Wait for the RunButton to reach a specific phase and take a snapshot.
 *
 * Phases correspond to the execution lifecycle:
 *   - "idle" — No execution, waiting for files
 *   - "uploading" — Files uploading to R2
 *   - "running" — Go engine executing the workflow
 *   - "completed" — Execution finished with results
 *   - "failed" — Execution errored
 *
 * Usage:
 * ```ts
 * await waitForPhase(page, "uploading", "my-test-02-uploading.png");
 * ```
 */
export async function waitForPhase(
  page: Page,
  phase: "idle" | "uploading" | "running" | "completed" | "failed",
  screenshotName?: string,
  options?: { timeout?: number },
) {
  const runButton = page.locator('[data-testid="run-button"]');
  await expect(runButton).toHaveAttribute("data-phase", phase, {
    timeout: options?.timeout ?? 30_000,
  });

  if (screenshotName) {
    await expect(page).toHaveScreenshot(screenshotName, SCREENSHOT_OPTIONS);
  }
}

/**
 * Try to capture a transient execution phase with a snapshot.
 *
 * Some phases (uploading, running) are fleeting — the execution may already
 * be past them by the time we check. This helper attempts to capture but
 * does NOT fail the test if the phase has already passed.
 *
 * Usage:
 * ```ts
 * await runButton.click();
 * await captureTransientPhase(page, ["uploading", "running"], "my-test-02-in-progress.png");
 * ```
 */
export async function captureTransientPhase(
  page: Page,
  phases: string[],
  screenshotName: string,
) {
  const runButton = page.locator('[data-testid="run-button"]');
  const currentPhase = await runButton.getAttribute("data-phase");
  if (currentPhase && phases.includes(currentPhase)) {
    await expect(page).toHaveScreenshot(screenshotName, SCREENSHOT_OPTIONS);
  }
}

/**
 * Snapshot the upload progress state.
 *
 * Upload file items expose `data-file-status` (pending | uploading | completed | failed).
 * This helper waits for at least one upload item to appear before capturing.
 *
 * Usage:
 * ```ts
 * await runButton.click();
 * await captureUploadProgress(page, "my-test-02-uploading.png");
 * ```
 */
export async function captureUploadProgress(
  page: Page,
  screenshotName: string,
) {
  const uploadItem = page.locator('[data-testid="upload-file"]').first();
  try {
    await expect(uploadItem).toBeVisible({ timeout: 5_000 });
    await expect(page).toHaveScreenshot(screenshotName, SCREENSHOT_OPTIONS);
  } catch {
    // Upload may already be complete — not a test failure
  }
}

// ---------------------------------------------------------------------------
// Download verification — prove the user can actually get their output
// ---------------------------------------------------------------------------

/**
 * Verify output files are listed in the results panel and downloads are ready.
 *
 * Checks that the expected number of output file rows are visible, each
 * showing a filename and file size. Waits for presigned download URLs to
 * load (spinners replaced by download icons) before returning.
 *
 * This ensures screenshots capture the "ready to download" state, not the
 * intermediate "loading presigned URLs" state with spinners.
 */
export async function assertOutputFiles(
  page: Page,
  expectedCount: number,
) {
  const results = page.locator('[data-testid="execution-results"]');
  await expect(results).toBeVisible({ timeout: 10_000 });

  // Verify the summary text
  const label = expectedCount === 1 ? "1 file ready" : `${expectedCount} files ready`;
  await expect(results.getByText(label)).toBeVisible();

  // Verify each output file row has a name and size
  const outputFiles = page.locator('[data-testid="output-file"]');
  await expect(outputFiles).toHaveCount(expectedCount, { timeout: 10_000 });

  // Wait for download buttons to be ready (presigned URLs loaded).
  // Single file: wait for data-testid="download-button" to appear.
  // Multi file: wait for data-testid="download-all-button" to be enabled.
  if (expectedCount === 1) {
    await expect(
      page.locator('[data-testid="download-button"]'),
    ).toBeVisible({ timeout: 15_000 });
  } else {
    await expect(
      page.locator('[data-testid="download-all-button"]'),
    ).toBeEnabled({ timeout: 15_000 });
  }

  return outputFiles;
}

/**
 * Click the download button and verify a file is actually received.
 *
 * For single-file results, clicks [data-testid="download-button"].
 * Waits for Playwright's download event, then verifies the file has
 * a suggested filename and non-zero size.
 *
 * Returns the Download object for further inspection if needed.
 */
export async function downloadAndVerify(
  page: Page,
): Promise<Download> {
  const downloadButton = page.locator('[data-testid="download-button"]');
  await expect(downloadButton).toBeVisible({ timeout: 15_000 });

  // Start waiting for download before clicking
  const [download] = await Promise.all([
    page.waitForEvent("download", { timeout: 30_000 }),
    downloadButton.click(),
  ]);

  // Verify the download has a filename
  const suggestedName = download.suggestedFilename();
  expect(suggestedName).toBeTruthy();

  // Save to temp path and verify non-zero size
  const filePath = await download.path();
  expect(filePath).toBeTruthy();
  const stats = fs.statSync(filePath!);
  expect(stats.size).toBeGreaterThan(0);

  console.log(
    `[download] Received: ${suggestedName} (${stats.size} bytes)`,
  );

  return download;
}

/**
 * Read the contents of a downloaded file.
 *
 * Playwright saves downloads to a temp directory. This reads the file
 * so tests can inspect the actual output — verify a compressed image
 * has valid PNG headers, a cleaned CSV has the right rows, etc.
 */
export async function readDownloadedFile(
  download: Download,
): Promise<Buffer> {
  const filePath = await download.path();
  if (!filePath) throw new Error("Download has no path — was it cancelled?");
  return fs.readFileSync(filePath);
}

/**
 * Verify a downloaded file has the expected image format by checking
 * magic bytes (file signature).
 *
 * Supports: PNG, JPEG, WebP, GIF.
 */
export function assertImageFormat(
  buffer: Buffer,
  expectedFormat: "png" | "jpeg" | "webp" | "gif",
) {
  const signatures: Record<string, number[]> = {
    png: [0x89, 0x50, 0x4e, 0x47],       // \x89PNG
    jpeg: [0xff, 0xd8, 0xff],              // JPEG SOI marker
    webp: [0x52, 0x49, 0x46, 0x46],        // RIFF (WebP container)
    gif: [0x47, 0x49, 0x46],               // GIF
  };
  const expected = signatures[expectedFormat];
  const actual = [...buffer.subarray(0, expected.length)];
  const matches = expected.every((byte, i) => actual[i] === byte);

  // For WebP, also check bytes 8-11 for "WEBP"
  if (expectedFormat === "webp" && matches) {
    const webpTag = buffer.subarray(8, 12).toString("ascii");
    expect(webpTag).toBe("WEBP");
    return;
  }

  if (!matches) {
    // Show diagnostic info about what we actually got
    const hex = [...buffer.subarray(0, 16)]
      .map((b) => b.toString(16).padStart(2, "0"))
      .join(" ");
    const preview = buffer.subarray(0, 200).toString("utf-8").replace(/[^\x20-\x7E]/g, ".");
    throw new Error(
      `Expected ${expectedFormat} format but got different bytes.\n` +
        `  First 16 bytes (hex): ${hex}\n` +
        `  Preview (ASCII): ${preview}\n` +
        `  File size: ${buffer.length} bytes`,
    );
  }
}

/**
 * Click "Download All" and verify files are received.
 *
 * For multi-file results, clicks [data-testid="download-all-button"].
 * Waits for at least one download event. Due to browser download
 * batching behavior, we verify at least the first file arrives.
 *
 * Returns the array of Download objects received.
 */
export async function downloadAllAndVerify(
  page: Page,
  expectedCount: number,
): Promise<Download[]> {
  const downloadAllButton = page.locator('[data-testid="download-all-button"]');
  await expect(downloadAllButton).toBeVisible({ timeout: 15_000 });
  await expect(downloadAllButton).toBeEnabled();

  // Collect downloads — the app triggers them sequentially
  const downloads: Download[] = [];
  const downloadPromise = new Promise<void>((resolve) => {
    const handler = (download: Download) => {
      downloads.push(download);
      if (downloads.length >= expectedCount) {
        page.removeListener("download", handler);
        resolve();
      }
    };
    page.on("download", handler);
    // Safety timeout — resolve with whatever we got after 30s
    setTimeout(() => {
      page.removeListener("download", handler);
      resolve();
    }, 30_000);
  });

  await downloadAllButton.click();
  await downloadPromise;

  // Verify we received at least 1 download (browsers may batch/block multiples)
  expect(downloads.length).toBeGreaterThanOrEqual(1);

  for (const download of downloads) {
    const name = download.suggestedFilename();
    const filePath = await download.path();
    expect(filePath).toBeTruthy();
    const stats = fs.statSync(filePath!);
    expect(stats.size).toBeGreaterThan(0);
    console.log(`[download] Received: ${name} (${stats.size} bytes)`);
  }

  return downloads;
}
