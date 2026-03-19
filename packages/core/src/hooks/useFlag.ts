"use client";

import { useState, useEffect } from "react";
import { isFlagEnabled, subscribeToFlags } from "../adapters/posthog/flagAdapter";

/**
 * Reactive boolean feature flag hook.
 *
 * Returns `true`/`false` once flags are loaded, `undefined` while loading.
 * Re-renders when flags are reloaded or updated.
 */
export function useFlag(key: string): boolean | undefined {
  const [value, setValue] = useState(() => isFlagEnabled(key));

  useEffect(() => {
    setValue(isFlagEnabled(key));
    return subscribeToFlags(() => setValue(isFlagEnabled(key)));
  }, [key]);

  return value;
}
