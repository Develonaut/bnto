import path from "path";
import fs from "fs";
import type { Page, Locator } from "@playwright/test";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  openConfigDialog,
  closeConfigDialog,
} from "../../helpers";

/** Browser execution journey — watermark-images */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

const OVERLAY_DIR = path.join(IMAGE_FIXTURES_DIR, "overlays");
const SCREENSHOTS_DIR = path.join(__dirname, "__screenshots__");

/** Upload an overlay image via the hidden file input inside the config dialog. */
async function uploadOverlay(page: Page, overlayPath: string) {
  await page.locator("#param-overlay").setInputFiles(overlayPath);
}

/** Set the overlay size slider to a target value using keyboard arrows. */
async function setOverlaySize(page: Page, target: number) {
  const thumb = page.locator('[data-testid="control-slider-param-size"] [role="slider"]');
  await thumb.focus();
  // Slider range is 1–500 (engine schema). Home goes to min (1).
  await thumb.press("Home");
  const stepsUp = target - 1;
  for (let i = 0; i < stepsUp; i++) await thumb.press("ArrowRight");
}

/** Screenshot preview with UI chrome hidden so only source + overlay remain. */
async function cleanPreviewShot(page: Page, preview: Locator): Promise<Buffer> {
  await page.evaluate(() => {
    const c = document.querySelector(
      '[data-testid="watermark-preview"] > div',
    ) as HTMLElement | null;
    if (c) {
      c.style.border = "none";
      c.style.borderRadius = "0";
    }
    document
      .querySelectorAll<HTMLElement>('[data-testid="watermark-preview"] [role="radiogroup"]')
      .forEach((el) => {
        el.style.visibility = "hidden";
      });
  });

  const shot = await preview.screenshot();

  await page.evaluate(() => {
    const c = document.querySelector(
      '[data-testid="watermark-preview"] > div',
    ) as HTMLElement | null;
    if (c) {
      c.style.border = "";
      c.style.borderRadius = "";
    }
    document
      .querySelectorAll<HTMLElement>('[data-testid="watermark-preview"] [role="radiogroup"]')
      .forEach((el) => {
        el.style.visibility = "";
      });
  });

  return shot;
}

/**
 * Direct pixel comparison between CSS preview and engine output.
 *
 * Does NOT use Playwright's toHaveScreenshot (which manages its own golden
 * files and breaks the comparison flow). Instead: render the engine output at
 * preview dimensions, take a screenshot, and compare the two screenshots
 * pixel-by-pixel via canvas. Saves preview, output, and diff images as
 * committed proof artifacts.
 */
async function comparePreviewToOutput(
  page: Page,
  previewShot: Buffer,
  outputBuf: Buffer,
  name: string,
  width: number,
  height: number,
) {
  // Render engine output in a container matching the preview dimensions
  const outputB64 = outputBuf.toString("base64");
  await page.evaluate(
    ({ src, w, h }) => {
      let el = document.getElementById("e2e-output-render");
      if (el) el.remove();
      el = document.createElement("div");
      el.id = "e2e-output-render";
      el.style.cssText = `position:fixed;top:0;left:0;width:${w}px;height:${h}px;z-index:99999;overflow:hidden;`;
      el.innerHTML = `<img src="${src}" style="width:100%;height:100%;object-fit:cover;" />`;
      document.body.appendChild(el);
    },
    { src: `data:image/jpeg;base64,${outputB64}`, w: width, h: height },
  );

  await page.waitForFunction(() => {
    const img = document.querySelector("#e2e-output-render img") as HTMLImageElement | null;
    return img?.complete && img.naturalWidth > 0;
  });

  const outputShot = await page.locator("#e2e-output-render").screenshot();
  await page.evaluate(() => document.getElementById("e2e-output-render")?.remove());

  // Canvas-based pixel comparison in the browser
  const previewB64 = previewShot.toString("base64");
  const outputShotB64 = outputShot.toString("base64");

  const { diffRatio, diffPngB64 } = await page.evaluate(
    async ({ a, b, w, h }) => {
      const load = (src: string) =>
        new Promise<HTMLImageElement>((res, rej) => {
          const img = new Image();
          img.onload = () => res(img);
          img.onerror = rej;
          img.src = `data:image/png;base64,${src}`;
        });

      const [imgA, imgB] = await Promise.all([load(a), load(b)]);

      const draw = (img: HTMLImageElement) => {
        const c = document.createElement("canvas");
        c.width = w;
        c.height = h;
        const ctx = c.getContext("2d")!;
        ctx.drawImage(img, 0, 0, w, h);
        return ctx.getImageData(0, 0, w, h);
      };

      const dA = draw(imgA);
      const dB = draw(imgB);

      // Build diff image: red = mismatch, faded original = match
      const diff = new ImageData(w, h);
      let diffCount = 0;
      // Per-channel threshold: JPEG artifacts cause small color shifts
      const thr = 30;

      for (let i = 0; i < dA.data.length; i += 4) {
        const dr = Math.abs(dA.data[i] - dB.data[i]);
        const dg = Math.abs(dA.data[i + 1] - dB.data[i + 1]);
        const db = Math.abs(dA.data[i + 2] - dB.data[i + 2]);

        if (dr > thr || dg > thr || db > thr) {
          diffCount++;
          diff.data[i] = 255;
          diff.data[i + 1] = 0;
          diff.data[i + 2] = 0;
          diff.data[i + 3] = 255;
        } else {
          diff.data[i] = dA.data[i];
          diff.data[i + 1] = dA.data[i + 1];
          diff.data[i + 2] = dA.data[i + 2];
          diff.data[i + 3] = 80;
        }
      }

      const dc = document.createElement("canvas");
      dc.width = w;
      dc.height = h;
      dc.getContext("2d")!.putImageData(diff, 0, 0);

      return {
        diffRatio: diffCount / (w * h),
        diffPngB64: dc.toDataURL("image/png").split(",")[1],
      };
    },
    { a: previewB64, b: outputShotB64, w: width, h: height },
  );

  // Save proof artifacts: preview, output, and diff images
  fs.mkdirSync(SCREENSHOTS_DIR, { recursive: true });
  fs.writeFileSync(path.join(SCREENSHOTS_DIR, `preview-${name}`), previewShot);
  fs.writeFileSync(path.join(SCREENSHOTS_DIR, `output-${name}`), outputShot);
  fs.writeFileSync(path.join(SCREENSHOTS_DIR, `diff-${name}`), Buffer.from(diffPngB64, "base64"));

  // JPEG artifacts cause ~1-2% pixel noise. Position errors with a 50% overlay
  // affect 10%+ of pixels. 3% tolerance catches position bugs cleanly.
  expect(
    diffRatio,
    `Preview vs engine diff for ${name}: ${(diffRatio * 100).toFixed(1)}% pixels differ (max 3%)`,
  ).toBeLessThan(0.03);
}

test.describe("watermark-images — preview @browser", () => {
  test("preview shows position grid and overlay appears on upload", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");
    await uploadFiles(page, [path.join(OVERLAY_DIR, "square-1x1.jpg")]);
    await openConfigDialog(page);

    await expect(page.getByTestId("watermark-preview")).toBeVisible();
    await expect(page.getByTestId("position-bottom-right")).toBeVisible();
    await expect(page.getByTestId("watermark-preview-overlay")).not.toBeVisible();

    await uploadOverlay(page, path.join(OVERLAY_DIR, "overlay-logo.png"));
    await expect(page.getByTestId("watermark-preview-overlay")).toBeVisible();

    await closeConfigDialog(page);
  });

  test("clicking position dots updates aria-checked", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");
    await uploadFiles(page, [path.join(OVERLAY_DIR, "square-1x1.jpg")]);
    await openConfigDialog(page);

    await expect(page.getByTestId("position-bottom-right")).toHaveAttribute("aria-checked", "true");

    await page.getByTestId("position-center").click();
    await expect(page.getByTestId("position-center")).toHaveAttribute("aria-checked", "true");
    await expect(page.getByTestId("position-bottom-right")).toHaveAttribute(
      "aria-checked",
      "false",
    );

    await page.getByTestId("position-top-left").click();
    await expect(page.getByTestId("position-top-left")).toHaveAttribute("aria-checked", "true");
    await expect(page.getByTestId("position-center")).toHaveAttribute("aria-checked", "false");

    await closeConfigDialog(page);
  });
});

test.describe("watermark-images — preview vs engine output @browser", () => {
  test("CSS preview matches engine composite at three positions", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");
    await uploadFiles(page, [path.join(OVERLAY_DIR, "square-1x1.jpg")]);
    await openConfigDialog(page);
    await uploadOverlay(page, path.join(OVERLAY_DIR, "overlay-logo.png"));
    await expect(page.getByTestId("watermark-preview-overlay")).toBeVisible();

    // Set overlay to 50% so positioning differences are clearly visible
    await setOverlaySize(page, 50);

    const preview = page.getByTestId("watermark-preview");
    const box = await preview.boundingBox();
    expect(box).not.toBeNull();
    const w = Math.round(box!.width);
    const h = Math.round(box!.height);

    // --- bottom-right (default) ---
    const shotBR = await cleanPreviewShot(page, preview);
    await closeConfigDialog(page);
    await runAndComplete(page);
    const bufBR = await downloadAndVerify(page, { magicBytes: MAGIC.JPEG });
    await comparePreviewToOutput(page, shotBR, bufBR, "composite-bottom-right.png", w, h);

    // --- center ---
    await openConfigDialog(page);
    await page.getByTestId("position-center").click();
    const shotC = await cleanPreviewShot(page, preview);
    await closeConfigDialog(page);
    const dl2 = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download2 = await dl2;
    const buf2 = fs.readFileSync((await download2.path())!);
    await comparePreviewToOutput(page, shotC, buf2, "composite-center.png", w, h);

    // --- top-left ---
    await openConfigDialog(page);
    await page.getByTestId("position-top-left").click();
    const shotTL = await cleanPreviewShot(page, preview);
    await closeConfigDialog(page);
    const dl3 = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download3 = await dl3;
    const buf3 = fs.readFileSync((await download3.path())!);
    await comparePreviewToOutput(page, shotTL, buf3, "composite-top-left.png", w, h);
  });
});

test.describe("watermark-images — engine output @browser", () => {
  test("full flow: upload, configure, run, verify JPEG", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");
    await uploadFiles(page, [path.join(OVERLAY_DIR, "square-1x1.jpg")]);

    await openConfigDialog(page);
    await uploadOverlay(page, path.join(OVERLAY_DIR, "overlay-logo.png"));
    await expect(page.getByTestId("watermark-preview-overlay")).toBeVisible();
    await closeConfigDialog(page);

    await runAndComplete(page);
    await expect(page.getByTestId("output-file")).toHaveCount(1);

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
    expect(buffer.length).toBeGreaterThan(1000);
  });

  test("different positions produce different output", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");
    await uploadFiles(page, [path.join(OVERLAY_DIR, "square-1x1.jpg")]);

    await openConfigDialog(page);
    await uploadOverlay(page, path.join(OVERLAY_DIR, "overlay-logo.png"));
    await expect(page.getByTestId("watermark-preview-overlay")).toBeVisible();
    await closeConfigDialog(page);

    await runAndComplete(page);
    const buf1 = await downloadAndVerify(page, { magicBytes: MAGIC.JPEG });

    await openConfigDialog(page);
    await page.getByTestId("position-top-left").click();
    await closeConfigDialog(page);

    const dl2 = page.waitForEvent("download");
    await runAndComplete(page, { expectStep: "completed" });
    const download2 = await dl2;
    const buf2 = fs.readFileSync((await download2.path())!);

    expect(Buffer.compare(buf1, buf2)).not.toBe(0);
  });

  test("run without overlay fails gracefully", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");
    await uploadFiles(page, [path.join(OVERLAY_DIR, "square-1x1.jpg")]);

    // Engine requires an overlay — running without one should fail, not crash
    await runAndComplete(page, { expectStep: "failed" });
  });
});
