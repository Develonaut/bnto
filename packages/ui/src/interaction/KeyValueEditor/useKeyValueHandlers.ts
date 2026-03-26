"use client";

import { useCallback } from "react";

import type { KeyValuePair } from "./toPairs";

/** Mutation helpers for key-value pair editing. */
export function useKeyValueHandlers(
  pairs: KeyValuePair[],
  emitChange: (next: KeyValuePair[]) => void,
) {
  const updatePair = useCallback(
    (index: number, field: "key" | "value", text: string) => {
      const next = [...pairs];
      next[index] = { ...next[index], [field]: text };
      emitChange(next);
    },
    [pairs, emitChange],
  );

  const removePair = useCallback(
    (index: number) => emitChange(pairs.filter((_, i) => i !== index)),
    [pairs, emitChange],
  );

  return { updatePair, removePair };
}
