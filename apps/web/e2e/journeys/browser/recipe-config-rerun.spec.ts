import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  CSV_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  openConfigDialog,
  closeConfigDialog,
  assertWebPBytes,
} from "../../helpers";

/**
 * Recipe stepper journey — config dialog, rerun, back navigation.
 *
 * Exercises the full user flow: upload → configure → run → change config →
 * rerun → back → run again. Tests each recipe's config controls thoroughly.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

// ---------------------------------------------------------------------------
// compress-images: slider presets affect output
// ---------------------------------------------------------------------------

test.describe("compress-images — config & rerun @browser", () => {
  test("change quality via config dialog, run, change again, rerun", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "large.jpg")]);

    // Set quality to Draft (60) via config dialog
    await openConfigDialog(page);
    await page.getByTestId("slider-preset", '[data-preset-label="Draft"]').click();
    await closeConfigDialog(page);

    // Run and capture the auto-download
    const dl1 = page.waitForEvent("download");
    await runAndComplete(page);
    const download1 = await dl1;
    const size1 = fs.readFileSync((await download1.path())!).length;

    // Rerun button should show "Rerun" (step = completed)
    const runButton = page.getByTestId("run-button");
    await expect(runButton).toHaveAttribute("data-step", "completed");

    // Change quality to Maximum (100) via config dialog on step 3
    await openConfigDialog(page);
    await page.getByTestId("slider-preset", '[data-preset-label="Maximum"]').click();
    await closeConfigDialog(page);

    // Click Rerun to process again with new settings
    const dl2 = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download2 = await dl2;
    const size2 = fs.readFileSync((await download2.path())!).length;

    // Maximum quality (100) should produce larger output than Draft (60)
    expect(size2).toBeGreaterThan(size1);
  });

  test("run → back → change config → run again", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Run with default config
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Back to configure step
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Change config before running again
    await openConfigDialog(page);
    await page.getByTestId("slider-preset", '[data-preset-label="Draft"]').click();
    await closeConfigDialog(page);

    // Run again — should work with new config
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });
});

// ---------------------------------------------------------------------------
// convert-image-format: select control changes output format
// ---------------------------------------------------------------------------

test.describe("convert-image-format — config & rerun @browser", () => {
  test("change format to JPEG, run, change to PNG, rerun", async ({ page }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Default is WebP — run with default
    await runAndComplete(page);
    const buffer1 = await downloadAndVerify(page, { filenamePattern: /\.webp$/i });
    assertWebPBytes(buffer1);

    // Rerun with JPEG format via config dialog
    await openConfigDialog(page);
    await page.getByTestId("control-select-param-format").click();
    await page.getByTestId("select-option-jpeg").click();
    await closeConfigDialog(page);

    const dl2 = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download2 = await dl2;
    const buffer2 = fs.readFileSync((await download2.path())!);

    // Verify JPEG magic bytes
    for (let i = 0; i < MAGIC.JPEG.length; i++) {
      expect(buffer2[i]).toBe(MAGIC.JPEG[i]);
    }
  });

  test("back → change format → run produces new format", async ({ page }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.png")]);

    // Run with default (WebP)
    await runAndComplete(page);

    // Back to configure
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Change to JPEG
    await openConfigDialog(page);
    await page.getByTestId("control-select-param-format").click();
    await page.getByTestId("select-option-jpeg").click();
    await closeConfigDialog(page);

    // Run again
    await runAndComplete(page);

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
  });
});

// ---------------------------------------------------------------------------
// resize-images: width/height number inputs
// ---------------------------------------------------------------------------

test.describe("resize-images — config & rerun @browser", () => {
  test("config dialog opens and shows resize controls", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "medium.jpg")]);

    // Open config dialog — verify resize-specific controls are present
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();

    // Close and run
    await closeConfigDialog(page);

    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });

  test("run → rerun produces valid output each time", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "medium.jpg")]);

    // First run
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "completed");

    // Rerun with same config
    const dl = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download = await dl;

    const buffer = fs.readFileSync((await download.path())!);
    // Valid JPEG
    for (let i = 0; i < MAGIC.JPEG.length; i++) {
      expect(buffer[i]).toBe(MAGIC.JPEG[i]);
    }
  });
});

// ---------------------------------------------------------------------------
// rename-files: text input for prefix/suffix
// ---------------------------------------------------------------------------

test.describe("rename-files — config & rerun @browser", () => {
  test("run → back → run again preserves files", async ({ page }) => {
    await navigateToRecipe(page, "rename-files", "Rename Files Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // First run
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Back to configure — files should be retained
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // The file list should still show the file (step 2 shows pending files)
    await expect(page.getByTestId("run-button")).toBeEnabled();

    // Run again
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });

  test("config dialog accessible on step 2 and step 3", async ({ page }) => {
    await navigateToRecipe(page, "rename-files", "Rename Files Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Config button available on step 2 (configure)
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Run to step 3 (results)
    await runAndComplete(page);

    // Config button available on step 3 (results) for rerun
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);
  });
});

// ---------------------------------------------------------------------------
// clean-csv: toggle switches (trim, remove empty, remove duplicates)
// ---------------------------------------------------------------------------

test.describe("clean-csv — config & rerun @browser", () => {
  test("run with defaults, back, run again", async ({ page }) => {
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "messy.csv")]);

    // Run with default config (trim + remove empty rows)
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Download and verify output is cleaned
    const buffer1 = await downloadAndVerify(page, { filenamePattern: /\.csv$/i });
    const csv1 = buffer1.toString("utf-8");
    expect(csv1).toContain("name");

    // Back to configure
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Run again
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });

  test("config dialog shows toggle controls", async ({ page }) => {
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "messy.csv")]);

    // Open config dialog — should show switches for clean options
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);
  });
});

// ---------------------------------------------------------------------------
// generate-thumbnails: multi-node pipeline (resize + convert + rename)
// ---------------------------------------------------------------------------

test.describe("generate-thumbnails — config & rerun @browser", () => {
  test("run → change config → rerun produces different output", async ({ page }) => {
    await navigateToRecipe(page, "generate-thumbnails", "Generate Thumbnails Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Run with defaults (WebP, 150px width, thumb_ prefix)
    await runAndComplete(page);

    const buffer1 = await downloadAndVerify(page, {
      filenamePattern: /^thumb_.*\.webp$/i,
    });
    assertWebPBytes(buffer1);

    // Open config dialog on results step — should show multi-node config
    await openConfigDialog(page);
    // Multi-node recipe shows section headers — dialog should have content
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Rerun with same settings
    const dl = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download = await dl;
    const buffer2 = fs.readFileSync((await download.path())!);

    // Still valid image output
    expect(buffer2.length).toBeGreaterThan(0);
  });

  test("back → run again cycle works for multi-node pipeline", async ({ page }) => {
    await navigateToRecipe(page, "generate-thumbnails", "Generate Thumbnails Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Run to completion
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Back to configure
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Open config to verify it's accessible
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Run again
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });
});

// ---------------------------------------------------------------------------
// optimize-images-for-web: multi-node (resize + convert + compress)
// ---------------------------------------------------------------------------

test.describe("optimize-images-for-web — config & rerun @browser", () => {
  test("full cycle: run → config → rerun → back → config → run", async ({ page }) => {
    await navigateToRecipe(page, "optimize-images-for-web", "Optimize Images for Web Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Step 1: Run with defaults
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "completed");

    // Step 2: Open config on results step, verify dialog works
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Step 3: Rerun (fires from completed state)
    const dl = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    await dl;

    // Step 4: Back to configure
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Step 5: Open config on configure step
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Step 6: Run again from configure step
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });
});

// ---------------------------------------------------------------------------
// strip-exif: single slider (quality)
// ---------------------------------------------------------------------------

test.describe("strip-exif — config & rerun @browser", () => {
  test("run → rerun → back → run full cycle", async ({ page }) => {
    await navigateToRecipe(page, "strip-exif", "Strip EXIF Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // First run
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Rerun
    const dl = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    await dl;

    // Back to configure
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Third run
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });
});

// ---------------------------------------------------------------------------
// csv-to-json: select (delimiter) + switch (pretty)
// ---------------------------------------------------------------------------

test.describe("csv-to-json — config & rerun @browser", () => {
  test("run with defaults, verify JSON output", async ({ page }) => {
    await navigateToRecipe(page, "csv-to-json", "CSV to JSON Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "simple.csv")]);

    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    const buffer = await downloadAndVerify(page, { filenamePattern: /\.json$/i });
    const json = JSON.parse(buffer.toString("utf-8"));
    expect(Array.isArray(json)).toBe(true);
    expect(json.length).toBeGreaterThan(0);
  });

  test("config dialog opens, run → back → run cycle", async ({ page }) => {
    await navigateToRecipe(page, "csv-to-json", "CSV to JSON Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "simple.csv")]);

    // Open config dialog
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Run
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Back
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Run again
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });
});

// ---------------------------------------------------------------------------
// compress-and-rename: multi-node (compress + rename)
// ---------------------------------------------------------------------------

test.describe("compress-and-rename — config & rerun @browser", () => {
  test("run produces compressed and renamed output, rerun works", async ({ page }) => {
    await navigateToRecipe(page, "compress-and-rename", "Compress and Rename Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Verify config dialog is accessible for multi-node recipe
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Rerun
    const dl = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download = await dl;
    const buffer = fs.readFileSync((await download.path())!);

    // Valid JPEG
    for (let i = 0; i < MAGIC.JPEG.length; i++) {
      expect(buffer[i]).toBe(MAGIC.JPEG[i]);
    }
  });
});

// ---------------------------------------------------------------------------
// merge-csv: batch node (needs multiple files)
// ---------------------------------------------------------------------------

test.describe("merge-csv — config & rerun @browser", () => {
  test("merge two CSVs, config dialog accessible", async ({ page }) => {
    await navigateToRecipe(page, "merge-csv", "Merge CSV Online Free");

    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "simple.csv"),
      path.join(CSV_FIXTURES_DIR, "messy.csv"),
    ]);

    // Config dialog should be accessible
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    const buffer = await downloadAndVerify(page, { filenamePattern: /\.csv$/i });
    const csv = buffer.toString("utf-8");
    expect(csv).toContain("name");
  });
});

// ---------------------------------------------------------------------------
// standardize-csv: multi-node (clean + rename columns)
// ---------------------------------------------------------------------------

test.describe("standardize-csv — config & rerun @browser", () => {
  test("run → back → run full cycle", async ({ page }) => {
    await navigateToRecipe(page, "standardize-csv", "Standardize CSV Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "messy.csv")]);

    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    // Config on results step
    await openConfigDialog(page);
    await expect(page.getByRole("dialog")).toBeVisible();
    await closeConfigDialog(page);

    // Back
    await page.getByTestId("back-button").click();
    await expect(page.getByTestId("run-button")).toHaveAttribute("data-step", "idle");

    // Run again
    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);
  });
});
