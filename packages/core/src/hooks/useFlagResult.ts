"use client";

import { useState, useEffect } from "react";
import { getFlagResult, subscribeToFlags } from "../adapters/posthog/flagAdapter";
import type { FlagResult } from "../types/flags";

/**
 * Reactive full flag result hook.
 *
 * Returns the complete FlagResult (enabled, variant, payload) once flags
 * are loaded, `undefined` while loading. Use this when you need the
 * payload or want PostHog to emit `$feature_flag_called` for experiments.
 */
export function useFlagResult(key: string): FlagResult | undefined {
  const [value, setValue] = useState(() => getFlagResult(key));

  useEffect(() => {
    setValue(getFlagResult(key));
    return subscribeToFlags(() => setValue(getFlagResult(key)));
  }, [key]);

  return value;
}
