"use client";

import {
  isFlagEnabled,
  getFlagVariant,
  getFlagResult,
  subscribeToFlags,
  reloadFlags,
} from "../adapters/posthog/flagAdapter";

/**
 * Flags client — public API for feature flag evaluation.
 *
 * Delegates directly to the PostHog flag adapter. No service layer needed —
 * flag reads are fire-and-forget with no mutations or cache.
 */
export function createFlagsClient() {
  return {
    isEnabled: isFlagEnabled,
    getVariant: getFlagVariant,
    getResult: getFlagResult,
    subscribe: subscribeToFlags,
    reload: reloadFlags,
  } as const;
}
