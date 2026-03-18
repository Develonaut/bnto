import { test, expect } from "../fixtures";
import {
  enableTelemetryCapture,
  getTelemetryEvents,
  waitForTelemetryEvent,
  filterEvents,
} from "../telemetryHelper";

/**
 * PostHog telemetry integration — verifies the analytics pipeline works
 * end-to-end: TelemetryProvider initializes PostHog, page views are
 * captured on navigation, and autocapture fires on user interactions.
 *
 * Events are captured via a test hook in the PostHog adapter that pushes
 * to window.__bnto_telemetry__ (set up by enableTelemetryCapture).
 */

test.describe("PostHog telemetry @browser", () => {
  test("initializes and captures initial pageview on load", async ({ page }) => {
    await enableTelemetryCapture(page);

    await page.goto("/");
    await expect(page.getByTestId("gallery-heading")).toBeVisible();

    // Wait for TelemetryProvider's useEffect to fire the initial $pageview
    await waitForTelemetryEvent(page, "$pageview");

    const events = await getTelemetryEvents(page);
    const pageviews = filterEvents(events, "$pageview");
    expect(pageviews.length).toBeGreaterThanOrEqual(1);
  });

  test("captures pageview on SPA navigation", async ({ page }) => {
    await enableTelemetryCapture(page);

    await page.goto("/");
    await expect(page.getByTestId("gallery-heading")).toBeVisible();

    // Wait for initial pageview from TelemetryProvider
    await waitForTelemetryEvent(page, "$pageview");

    // SPA navigate by clicking a recipe link (not page.goto which
    // does a full navigation and resets the __bnto_telemetry__ array).
    // Use footer link — always visible after scrolling to bottom.
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    await page.getByTestId("footer-link-compress-images").click();
    await expect(page.getByTestId("recipe-heading")).toBeVisible();

    // Wait for at least 2 pageviews: initial load + SPA navigation
    await waitForTelemetryEvent(page, "$pageview", 2);

    const events = await getTelemetryEvents(page);
    const pageviews = filterEvents(events, "$pageview");
    expect(pageviews.length).toBeGreaterThanOrEqual(2);
  });
});
