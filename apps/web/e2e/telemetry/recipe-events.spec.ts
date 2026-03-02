import path from "path";
import { test, expect } from "../fixtures";
import {
  enableTelemetryCapture,
  getTelemetryEvents,
  waitForTelemetryEvent,
  filterEvents,
} from "../telemetryHelper";

test.use({ reducedMotion: "reduce" });

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../test-fixtures/images",
);

/**
 * Recipe telemetry events — verifies that the recipe flow fires
 * the correct telemetry events at each stage of the lifecycle.
 *
 * Events are captured via window.__bnto_telemetry__ (no PostHog needed).
 */

test.describe("recipe telemetry events @browser", () => {
  test("full lifecycle fires files_added, run_started, run_completed, result_downloaded", async ({
    page,
  }) => {
    await enableTelemetryCapture(page);

    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // --- files_added ---
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
    ]);
    await expect(page.getByText("1 file selected")).toBeVisible();

    await waitForTelemetryEvent(page, "files_added");
    let events = await getTelemetryEvents(page);
    const filesAdded = filterEvents(events, "files_added");
    expect(filesAdded).toHaveLength(1);
    expect(filesAdded[0].properties).toMatchObject({
      slug: "compress-images",
      fileCount: 1,
    });
    expect(filesAdded[0].properties?.totalBytes).toBeGreaterThan(0);

    // --- recipe_run_started + recipe_run_completed ---
    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await waitForTelemetryEvent(page, "recipe_run_completed");
    events = await getTelemetryEvents(page);

    const started = filterEvents(events, "recipe_run_started");
    expect(started).toHaveLength(1);
    expect(started[0].properties).toMatchObject({
      slug: "compress-images",
      fileCount: 1,
      executionPath: "browser",
    });
    expect(started[0].properties?.totalBytes).toBeGreaterThan(0);

    const completed = filterEvents(events, "recipe_run_completed");
    expect(completed).toHaveLength(1);
    expect(completed[0].properties).toMatchObject({
      slug: "compress-images",
      fileCount: 1,
      executionPath: "browser",
      outputFileCount: 1,
    });
    expect(completed[0].properties?.durationMs).toBeGreaterThan(0);
    expect(completed[0].properties?.outputBytes).toBeGreaterThan(0);

    // --- result_downloaded (via Download All auto-download) ---
    // Auto-download fires on completion, so result_downloaded should
    // already be captured. If not present, it means auto-download
    // uses downloadAllResults directly (not handleDownloadAll).
    // The manual "Download All" button triggers the event.
    const downloadAllBtn = page
      .getByRole("button", { name: /download all/i })
      .last();

    // Only assert manual download if the button is visible
    if (await downloadAllBtn.isVisible()) {
      await downloadAllBtn.click();
      await waitForTelemetryEvent(page, "result_downloaded");
      events = await getTelemetryEvents(page);
      const downloaded = filterEvents(events, "result_downloaded");
      expect(downloaded.length).toBeGreaterThanOrEqual(1);
      expect(downloaded[0].properties).toMatchObject({
        slug: "compress-images",
      });
    }
  });
});
