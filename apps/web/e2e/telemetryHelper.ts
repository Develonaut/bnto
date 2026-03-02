import type { Page } from "@playwright/test";

/**
 * Captured telemetry event from the adapter's test hook.
 */
export interface CapturedEvent {
  event: string;
  properties?: Record<string, unknown>;
}

/**
 * Enable telemetry event capture for E2E assertions.
 *
 * Must be called BEFORE navigating to any page. Sets up a global array
 * on `window.__bnto_telemetry__` that the PostHog adapter pushes events
 * into when `captureEvent()` is called.
 *
 * This approach is more reliable than network interception because
 * PostHog JS uses gzip-compressed requests to `/i/v0/e/` which are
 * hard to intercept and parse from Playwright.
 *
 * Usage:
 *   await enableTelemetryCapture(page);
 *   await page.goto("/compress-images");
 *   // ... do things ...
 *   const events = await getTelemetryEvents(page);
 *   expect(events.filter(e => e.event === "$pageview").length).toBeGreaterThan(0);
 */
export async function enableTelemetryCapture(page: Page) {
  // Initialize the global array BEFORE the page loads so the adapter
  // sees it immediately when captureEvent() fires.
  await page.addInitScript(() => {
    (window as unknown as Record<string, unknown>).__bnto_telemetry__ = [];
  });
}

/**
 * Read all captured telemetry events from the page.
 */
export async function getTelemetryEvents(page: Page) {
  return page.evaluate(() => {
    const events = (window as unknown as Record<string, unknown>).__bnto_telemetry__;
    return (Array.isArray(events) ? events : []) as Array<{
      event: string;
      properties?: Record<string, unknown>;
    }>;
  });
}

/**
 * Wait until at least `count` events with the given name appear.
 *
 * Uses `page.waitForFunction` instead of a raw timeout — the assertion
 * resolves as soon as the events exist, or fails after Playwright's
 * default timeout.
 */
export async function waitForTelemetryEvent(
  page: Page,
  eventName: string,
  count = 1,
) {
  await page.waitForFunction(
    ({ name, min }) => {
      const events = (window as unknown as Record<string, unknown>).__bnto_telemetry__;
      if (!Array.isArray(events)) return false;
      return events.filter((e: { event: string }) => e.event === name).length >= min;
    },
    { name: eventName, min: count },
  );
}

/**
 * Filter captured events by event name.
 */
export function filterEvents(events: CapturedEvent[], eventName: string) {
  return events.filter((e) => e.event === eventName);
}
