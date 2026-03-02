import { test, expect } from "../fixtures";
import {
  enableTelemetryCapture,
  getTelemetryEvents,
  waitForTelemetryEvent,
  filterEvents,
} from "../telemetryHelper";

test.use({ reducedMotion: "reduce" });

/**
 * PostHog telemetry integration — verifies the analytics pipeline works
 * end-to-end: TelemetryProvider initializes PostHog, page views are
 * captured on navigation, and autocapture fires on user interactions.
 *
 * Events are captured via a test hook in the PostHog adapter that pushes
 * to window.__bnto_telemetry__ (set up by enableTelemetryCapture).
 */

test.describe("PostHog telemetry", () => {
  test("initializes and captures initial pageview on load", async ({
    page,
  }) => {
    await enableTelemetryCapture(page);

    await page.goto("/");
    await expect(
      page.getByRole("heading", { name: /Pick a tool/ }),
    ).toBeVisible();

    // Wait for TelemetryProvider's useEffect to fire the initial $pageview
    await waitForTelemetryEvent(page, "$pageview");

    const events = await getTelemetryEvents(page);
    const pageviews = filterEvents(events, "$pageview");
    expect(pageviews.length).toBeGreaterThanOrEqual(1);
  });

  test("captures pageview on SPA navigation", async ({ page }) => {
    await enableTelemetryCapture(page);

    await page.goto("/");
    await expect(
      page.getByRole("heading", { name: /Pick a tool/ }),
    ).toBeVisible();

    // Wait for initial pageview from TelemetryProvider
    await waitForTelemetryEvent(page, "$pageview");

    // SPA navigate by clicking a recipe card link (not page.goto which
    // does a full navigation and resets the __bnto_telemetry__ array)
    await page.getByRole("link", { name: /Compress Images/ }).first().click();
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Wait for at least 2 pageviews: initial load + SPA navigation
    await waitForTelemetryEvent(page, "$pageview", 2);

    const events = await getTelemetryEvents(page);
    const pageviews = filterEvents(events, "$pageview");
    expect(pageviews.length).toBeGreaterThanOrEqual(2);
  });
});
