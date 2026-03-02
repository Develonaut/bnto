"use client";

import {
  initPostHog,
  captureEvent,
  identifyUser,
  resetUser,
  isInitialized,
} from "../adapters/posthog/posthogAdapter";

/**
 * Telemetry client — public API for event tracking, user identification, and reset.
 *
 * Delegates directly to the PostHog adapter. No service layer needed —
 * telemetry is fire-and-forget with no queries, mutations, or cache.
 */
export function createTelemetryClient() {
  return {
    init: initPostHog,
    capture: captureEvent,
    identify: identifyUser,
    reset: resetUser,
    isInitialized,
  } as const;
}

export type TelemetryClient = ReturnType<typeof createTelemetryClient>;
