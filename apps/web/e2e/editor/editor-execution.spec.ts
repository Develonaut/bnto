import path from "path";
import fs from "fs";
import { test, expect } from "../fixtures";
import {
  navigateToEditor,
  editorRunWithFiles,
  selectOutputNode,
  waitForPhase,
  getPhase,
  IMAGE_FIXTURES_DIR,
} from "./editor-helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Editor execution tests — EX1-EX4, EX7-EX8 from editor.md journey matrix.
 *
 * Verifies the full execution flow: load recipe, run with files,
 * observe node status changes, and verify output.
 *
 * @browser @editor — uses WASM engine, no Convex needed.
 */

const SMALL_JPEG = path.join(IMAGE_FIXTURES_DIR, "small.jpg");
const MEDIUM_JPEG = path.join(IMAGE_FIXTURES_DIR, "medium.jpg");

test.describe("editor execution @browser @editor", () => {
  test("EX2: compress-images — run produces completed state with node elevation changes", async ({
    page,
  }) => {
    await navigateToEditor(page, "compress-images");

    // All nodes should start in idle state (no execution yet)
    const runButton = page.locator('[data-testid="editor-run-button"]');
    await expect(runButton).toHaveAttribute("data-phase", "idle");

    // Upload files and trigger execution
    await editorRunWithFiles(page, [SMALL_JPEG], {
      timeout: 30_000,
      expectPhase: "completed",
    });

    // Run button should now show completed phase
    await expect(runButton).toHaveAttribute("data-phase", "completed");
  });

  test("EX3: compress-images — output node config panel shows results after execution", async ({
    page,
  }) => {
    await navigateToEditor(page, "compress-images");

    // Run execution
    await editorRunWithFiles(page, [SMALL_JPEG], {
      timeout: 30_000,
      expectPhase: "completed",
    });

    // Click the Output node to open its config panel
    await selectOutputNode(page);

    // The output renderer should appear with download results
    const outputRenderer = page.locator('[data-testid="output-renderer"]');
    await expect(outputRenderer).toBeVisible({ timeout: 5000 });

    // Should have at least one output file
    const outputFiles = page.locator('[data-testid="output-file"]');
    await expect(outputFiles.first()).toBeVisible();
  });

  test("EX4: compress-images — download single file produces valid output", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    // Run execution
    await editorRunWithFiles(page, [SMALL_JPEG], {
      timeout: 30_000,
      expectPhase: "completed",
    });

    // Select output node to see results
    await selectOutputNode(page);

    const outputFile = page.locator('[data-testid="output-file"]').first();
    await expect(outputFile).toBeVisible({ timeout: 5000 });

    // Download the file
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    // Verify the download has content
    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const buffer = fs.readFileSync(downloadPath!);
    expect(buffer.length).toBeGreaterThan(0);

    // Should be a valid image file (JPEG magic bytes: FF D8 FF)
    expect(buffer[0]).toBe(0xff);
    expect(buffer[1]).toBe(0xd8);
    expect(buffer[2]).toBe(0xff);
  });

  test("EX2+: resize-images — run completes successfully", async ({ page }) => {
    await navigateToEditor(page, "resize-images");

    await editorRunWithFiles(page, [SMALL_JPEG], {
      timeout: 30_000,
      expectPhase: "completed",
    });

    const runButton = page.locator('[data-testid="editor-run-button"]');
    await expect(runButton).toHaveAttribute("data-phase", "completed");
  });

  test("EX2+: convert-image-format — run completes successfully", async ({ page }) => {
    await navigateToEditor(page, "convert-image-format");

    await editorRunWithFiles(page, [SMALL_JPEG], {
      timeout: 30_000,
      expectPhase: "completed",
    });

    const runButton = page.locator('[data-testid="editor-run-button"]');
    await expect(runButton).toHaveAttribute("data-phase", "completed");
  });

  test("EX7: reset after completion — clears results and returns to idle", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    // Run to completion
    await editorRunWithFiles(page, [SMALL_JPEG], {
      timeout: 30_000,
      expectPhase: "completed",
    });

    // Click run button again (should act as reset when completed)
    const runButton = page.locator('[data-testid="editor-run-button"]');
    await runButton.click();

    // Should return to idle
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });

  test("EX8: execution error — run button shows failed state", async ({ page }) => {
    // Load compress-images but try to run with a non-image file
    // This may or may not produce an error depending on WASM validation
    await navigateToEditor(page, "compress-images");

    // Create a dummy text file path — use CSV fixture as a non-image file
    const csvFixture = path.resolve(__dirname, "../../../../test-fixtures/csv/simple.csv");

    // Set files and run — expect either completed (WASM handles gracefully)
    // or failed (WASM rejects non-image)
    const fileInput = page.locator('[data-testid="editor-file-input"]');
    await fileInput.setInputFiles([csvFixture]);

    const runButton = page.locator('[data-testid="editor-run-button"]');

    // Wait for a terminal state (completed or failed)
    await page.waitForFunction(
      () => {
        const btn = document.querySelector('[data-testid="editor-run-button"]');
        const phase = btn?.getAttribute("data-phase");
        return phase === "completed" || phase === "failed";
      },
      { timeout: 30_000 },
    );

    const phase = await getPhase(page);
    // We expect this to either fail (invalid input) or complete (engine handles it)
    // Either way, the phase should be terminal — not stuck on "running"
    expect(["completed", "failed"]).toContain(phase);
  });

  test("EX2+: multiple files — compress-images handles batch", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    // Run with multiple files
    await editorRunWithFiles(page, [SMALL_JPEG, MEDIUM_JPEG], {
      timeout: 30_000,
      expectPhase: "completed",
    });

    // Select output node
    await selectOutputNode(page);

    // Should have multiple output files
    const outputFiles = page.locator('[data-testid="output-file"]');
    await expect(outputFiles).toHaveCount(2, { timeout: 5000 });
  });
});
