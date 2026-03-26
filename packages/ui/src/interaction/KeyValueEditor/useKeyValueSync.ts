"use client";

import { useState, useEffect } from "react";

import type { KeyValuePair } from "./toPairs";
import { toPairs } from "./toPairs";
import { toRecord } from "./toRecord";

/** Sync internal pairs state when external value changes. */
export function useKeyValueSync(value: Record<string, string>) {
  const [pairs, setPairs] = useState<KeyValuePair[]>(() => toPairs(value));

  useEffect(() => {
    const currentRecord = toRecord(pairs);
    const isSame =
      Object.keys(currentRecord).length === Object.keys(value).length &&
      Object.entries(currentRecord).every(([k, v]) => value[k] === v);
    if (!isSame) setPairs(toPairs(value));
    // Intentionally only react to external `value` changes -- pairs are derived from value.
  }, [value]);

  return [pairs, setPairs] as const;
}
