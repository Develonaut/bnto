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
