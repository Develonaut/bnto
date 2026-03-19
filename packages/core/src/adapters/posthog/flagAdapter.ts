"use client";

/**
 * Feature flag adapter — wraps PostHog feature flag API.
 *
 * SSR-safe: all functions return undefined/false on the server.
 * Uses the shared PostHog instance from window.__bnto_ph__.
 */

import type { FlagResult, FlagSubscriber } from "../../types/flags";
import { getSharedInstance } from "./shared";

/** Window-level key for E2E test flag assertions. */
const FLAGS_TEST_KEY = "__bnto_flags__" as const;

/** Push a flag evaluation to the E2E test hook (if present). */
function pushTestHook(key: string, value: unknown) {
  if (typeof window === "undefined") return;
  const w = window as unknown as Record<string, unknown>;
  if (Array.isArray(w[FLAGS_TEST_KEY])) {
    (w[FLAGS_TEST_KEY] as Array<{ key: string; value: unknown }>).push({ key, value });
  }
}

/** Check if a boolean feature flag is enabled. */
export function isFlagEnabled(key: string): boolean | undefined {
  const ph = getSharedInstance();
  if (!ph) return undefined;
  const result = ph.isFeatureEnabled(key);
  pushTestHook(key, result);
  return result ?? undefined;
}

/** Get the variant key for a multivariate flag. */
export function getFlagVariant(key: string): string | boolean | undefined {
  const ph = getSharedInstance();
  if (!ph) return undefined;
  const result = ph.getFeatureFlag(key);
  pushTestHook(key, result);
  return result ?? undefined;
}

/**
 * Get the full flag result including payload.
 *
 * Preferred over isFlagEnabled/getFlagVariant when you need
 * the payload or when experiment tracking is important —
 * PostHog emits `$feature_flag_called` on this call.
 */
export function getFlagResult(key: string): FlagResult | undefined {
  const ph = getSharedInstance();
  if (!ph) return undefined;
  const variant = ph.getFeatureFlag(key);
  const payload = ph.getFeatureFlagPayload(key);
  const enabled = ph.isFeatureEnabled(key);
  if (variant === undefined && enabled === undefined) return undefined;
  const result: FlagResult = {
    key,
    enabled: enabled ?? false,
    variant: variant ?? undefined,
    payload,
  };
  pushTestHook(key, result);
  return result;
}

/**
 * Subscribe to flag loading/updates. Callback fires when PostHog
 * loads flags (initial) or when flags are reloaded.
 *
 * Returns an unsubscribe function.
 */
export function subscribeToFlags(callback: FlagSubscriber): () => void {
  const ph = getSharedInstance();
  if (!ph) return () => {};
  return ph.onFeatureFlags(callback);
}

/** Force-reload flags from PostHog servers. */
export function reloadFlags() {
  const ph = getSharedInstance();
  if (!ph) return;
  ph.reloadFeatureFlags();
}
