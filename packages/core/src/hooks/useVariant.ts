"use client";

import { useState, useEffect } from "react";
import { getFlagVariant, subscribeToFlags } from "../adapters/posthog/flagAdapter";

/**
 * Reactive variant key hook for multivariate flags.
 *
 * Returns the variant string (e.g., "control", "test-a") once flags are loaded,
 * `undefined` while loading. For boolean flags, returns `true`/`false`.
 */
export function useVariant(key: string): string | boolean | undefined {
  const [value, setValue] = useState(() => getFlagVariant(key));

  useEffect(() => {
    setValue(getFlagVariant(key));
    return subscribeToFlags(() => setValue(getFlagVariant(key)));
  }, [key]);

  return value;
}
