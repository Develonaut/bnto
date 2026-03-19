"use client";

/**
 * Shared PostHog instance accessor.
 *
 * Both the telemetry adapter and the flag adapter read from the same
 * window-level PostHog instance. This avoids cross-imports between adapters.
 */

import type { PostHog } from "posthog-js";

/** Window-level key for the initialized PostHog instance. */
export const PH_KEY = "__bnto_ph__" as const;

/** Type-safe accessor for the shared PostHog instance on window. */
export function getSharedInstance(): PostHog | undefined {
  if (typeof window === "undefined") return undefined;
  return (window as unknown as Record<string, unknown>)[PH_KEY] as PostHog | undefined;
}
