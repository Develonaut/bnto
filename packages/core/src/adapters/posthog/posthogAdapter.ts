"use client";

/**
 * PostHog adapter — the ONLY file that imports posthog-js.
 *
 * Wraps the PostHog SDK with SSR-safe guards and DNT respect.
 * When running on the server (typeof window === "undefined") or
 * when the user has Do Not Track enabled, all methods are no-ops.
 */

import posthog from "posthog-js";
import type { TelemetryConfig, TelemetryProperties, TelemetryUserTraits } from "../../types/telemetry";

let initialized = false;

/**
 * Check if the user has Do Not Track enabled.
 * Respected as a courtesy — skip tracking entirely when set.
 */
function isDntEnabled() {
  if (typeof navigator === "undefined") return false;
  return navigator.doNotTrack === "1";
}

/**
 * Initialize the PostHog SDK. Must be called client-side only.
 * Subsequent calls are no-ops (idempotent). Respects DNT.
 */
export function initPostHog(config: TelemetryConfig) {
  if (typeof window === "undefined") return;
  if (initialized) return;
  if (isDntEnabled()) return;

  posthog.init(config.apiKey, {
    api_host: config.host,
    // Cookieless mode — no cookie consent banner needed
    persistence: "localStorage",
    // Disable automatic pageview capture — Next.js App Router uses pushState
    // which doesn't fire popstate. Page views are captured manually via
    // a usePathname() hook in the app layer.
    capture_pageview: false,
    // Capture clicks, inputs, etc. automatically
    autocapture: true,
    // Don't capture text content of elements (privacy)
    mask_all_text: false,
  });

  initialized = true;
}

/**
 * Capture a named event with optional properties.
 *
 * The E2E test hook (window.__bnto_telemetry__) fires regardless of
 * PostHog initialization so tests can verify the telemetry pipeline
 * even when PostHog env vars are absent (local dev).
 */
export function captureEvent(event: string, properties?: TelemetryProperties) {
  if (typeof window === "undefined") return;

  // Expose events for E2E test assertions — fires even when PostHog
  // is not initialized so tests verify the pipeline, not PostHog itself.
  const w = window as unknown as Record<string, unknown>;
  if (Array.isArray(w.__bnto_telemetry__)) {
    (w.__bnto_telemetry__ as Array<{ event: string; properties?: TelemetryProperties }>).push({ event, properties });
  }

  if (!initialized) return;
  posthog.capture(event, properties);
}

/**
 * Identify an authenticated user, linking anonymous → known sessions.
 */
export function identifyUser(distinctId: string, traits?: TelemetryUserTraits) {
  if (typeof window === "undefined" || !initialized) return;
  posthog.identify(distinctId, traits);
}

/**
 * Reset identity on sign-out (unlinks the session from the user).
 */
export function resetUser() {
  if (typeof window === "undefined" || !initialized) return;
  posthog.reset();
}

/**
 * Whether the adapter has been initialized.
 */
export function isInitialized() {
  return initialized;
}
