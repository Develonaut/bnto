import path from "path";
import { test, expect } from "../fixtures";
import {
  enableTelemetryCapture,
  getTelemetryEvents,
  waitForTelemetryEvent,
  filterEvents,
} from "../telemetryHelper";

const FIXTURES_DIR = path.resolve(__dirname, "../../../../test-fixtures/images");

/**
 * Recipe telemetry events — verifies that the recipe flow fires
 * the correct telemetry events at each stage of the lifecycle.
 *
 * Events are captured via window.__bnto_telemetry__ (no PostHog needed).
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("recipe telemetry events @browser", () => {
  test("full lifecycle fires files_added, run_started, run_completed, result_downloaded", async ({
    page,
  }) => {
    await enableTelemetryCapture(page);

    await page.goto("/compress-images");
    await expect(page.getByTestId("recipe-heading")).toBeVisible();

    // --- files_added ---
    const fileInput = page.getByTestId("file-input");
    await fileInput.setInputFiles([path.join(FIXTURES_DIR, "small.jpg")]);
    await expect(page.getByTestId("run-button")).toBeVisible();

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
    const runButton = page.getByTestId("run-button", ":visible");
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-step", "completed", {
      timeout: 30000,
    });

    await waitForTelemetryEvent(page, "recipe_run_completed");
    events = await getTelemetryEvents(page);

    const started = filterEvents(events, "recipe_run_started");
    expect(started).toHaveLength(1);
    expect(started[0].properties).toMatchObject({
      slug: "compress-images",
      fileCount: 1,
    });
    expect(started[0].properties?.totalBytes).toBeGreaterThan(0);

    const completed = filterEvents(events, "recipe_run_completed");
    expect(completed).toHaveLength(1);
    expect(completed[0].properties).toMatchObject({
      slug: "compress-images",
      fileCount: 1,
      outputFileCount: 1,
    });
    expect(completed[0].properties?.durationMs).toBeGreaterThan(0);
    expect(completed[0].properties?.outputBytes).toBeGreaterThan(0);

    // --- result_downloaded (the automatic download on completion) ---
    // A successful run downloads its results without a click, and that
    // download is recorded too.
    await waitForTelemetryEvent(page, "result_downloaded");
    events = await getTelemetryEvents(page);
    const downloaded = filterEvents(events, "result_downloaded");
    expect(downloaded).toHaveLength(1);
    expect(downloaded[0].properties).toEqual({
      slug: "compress-images",
      fileCount: 1,
      trigger: "auto",
    });
  });
});
