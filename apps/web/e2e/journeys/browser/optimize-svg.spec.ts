import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  VECTOR_FIXTURES_DIR,
  IMAGE_FIXTURES_DIR,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  runAndCaptureAutoDownload,
  assertValidSvg,
} from "../../helpers";

/**
 * Browser execution journey — optimize-svg
 *
 * Tests SVG optimization running 100% client-side via Rust→WASM.
 * Removes editor metadata, comments, unused namespaces, collapses
 * empty groups, and minifies — output is valid, smaller SVG.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("optimize-svg — browser execution @browser", () => {
  test("single SVG: optimize verbose fixture, verify size reduction", async ({ page }) => {
    await navigateToRecipe(page, "optimize-svg");

    const inputPath = path.join(VECTOR_FIXTURES_DIR, "verbose.svg");
    const inputSize = fs.statSync(inputPath).size;

    await uploadFiles(page, [inputPath]);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.svg$/i,
      maxSize: inputSize,
    });

    const svg = assertValidSvg(buffer);

    // Editor metadata stripped
    expect(svg).not.toContain("inkscape");
    expect(svg).not.toContain("sodipodi");
    expect(svg).not.toContain("<metadata");
    expect(svg).not.toContain("<!--");
  });

  test("single SVG: small clean SVG roundtrips correctly", async ({ page }) => {
    await navigateToRecipe(page, "optimize-svg");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.svg")]);

    await runAndComplete(page);

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.svg$/i,
    });

    const svg = assertValidSvg(buffer);

    // Visual elements survive optimization
    expect(svg).toContain("<rect");
    expect(svg).toContain("<circle");
  });

  test("batch: multiple SVGs auto-download as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "optimize-svg");

    await uploadFiles(page, [
      path.join(VECTOR_FIXTURES_DIR, "verbose.svg"),
      path.join(IMAGE_FIXTURES_DIR, "small.svg"),
    ]);

    const { download } = await runAndCaptureAutoDownload(page);
    expect(download.suggestedFilename()).toBe("optimize-svg-results.zip");

    await expect(page.getByTestId("output-file")).toHaveCount(2);
  });
});
