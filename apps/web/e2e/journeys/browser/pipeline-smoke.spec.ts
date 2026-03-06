import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  CSV_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  assertBrowserExecution,
  uploadFiles,
  runAndComplete,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Pipeline smoke test — all 6 Tier 1 bntos
 *
 * Post-refactor regression guard: verifies every recipe page still processes
 * files correctly after the processFiles → executePipeline migration
 * (Sprint 4H). Single file upload → run → output count → download for each.
 *
 * Not a deep functional test — each recipe has its own dedicated spec for
 * that. This is a quick end-to-end sanity check across all 6 recipes.
 */

const RECIPES = [
  {
    slug: "compress-images",
    h1: "Compress Images Online Free",
    fixture: path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    filenamePattern: /\.jpe?g$/i,
    magicBytes: MAGIC.JPEG,
  },
  {
    slug: "resize-images",
    h1: "Resize Images Online Free",
    fixture: path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    filenamePattern: /\.jpe?g$/i,
    magicBytes: MAGIC.JPEG,
  },
  {
    slug: "convert-image-format",
    h1: "Convert Image Format Online Free",
    fixture: path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    filenamePattern: /\.webp$/i,
    // WebP has split magic bytes — checked manually below
  },
  {
    slug: "clean-csv",
    h1: "Clean CSV Online Free",
    fixture: path.join(CSV_FIXTURES_DIR, "simple.csv"),
    filenamePattern: /\.csv$/i,
  },
  {
    slug: "rename-csv-columns",
    h1: "Rename CSV Columns Online Free",
    fixture: path.join(CSV_FIXTURES_DIR, "simple.csv"),
    filenamePattern: /\.csv$/i,
  },
  {
    slug: "rename-files",
    h1: "Rename Files Online Free",
    fixture: path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    // Renamed file keeps original format
    filenamePattern: /renamed/i,
  },
] as const;

test.describe("pipeline smoke — all 6 bntos @browser", () => {
  for (const recipe of RECIPES) {
    test(`${recipe.slug}: upload → run → download`, async ({ page }) => {
      await navigateToRecipe(page, recipe.slug, recipe.h1);
      await assertBrowserExecution(page);

      await uploadFiles(page, [recipe.fixture]);
      await runAndComplete(page);

      // At least one output file produced
      const outputFile = page.locator('[data-testid="output-file"]');
      await expect(outputFile).toHaveCount(1);

      // Download and verify filename pattern
      const downloadPromise = page.waitForEvent("download");
      await outputFile.getByRole("button", { name: /download/i }).click();
      const download = await downloadPromise;

      expect(download.suggestedFilename()).toMatch(recipe.filenamePattern);

      // Verify file is non-empty and has correct magic bytes
      const downloadPath = await download.path();
      expect(downloadPath).toBeTruthy();

      const buffer = fs.readFileSync(downloadPath!);
      expect(buffer.length).toBeGreaterThan(0);

      if ("magicBytes" in recipe && recipe.magicBytes) {
        for (let i = 0; i < recipe.magicBytes.length; i++) {
          expect(buffer[i]).toBe(recipe.magicBytes[i]);
        }
      }

      // WebP needs split magic byte check (RIFF at 0-3, WEBP at 8-11)
      if (recipe.slug === "convert-image-format") {
        for (let i = 0; i < MAGIC.WEBP_RIFF.length; i++) {
          expect(buffer[i]).toBe(MAGIC.WEBP_RIFF[i]);
        }
        for (let i = 0; i < MAGIC.WEBP_TAG.length; i++) {
          expect(buffer[8 + i]).toBe(MAGIC.WEBP_TAG[i]);
        }
      }
    });
  }
});
