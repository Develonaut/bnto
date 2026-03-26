"use client";

import { useCallback } from "react";

import { toRecord } from "./toRecord";
import { useKeyValueSync } from "./useKeyValueSync";
import { useKeyValueHandlers } from "./useKeyValueHandlers";

interface UseKeyValueEditorOptions {
  value: Record<string, string>;
  onChange: (record: Record<string, string>) => void;
  max?: number;
}

/** State and handlers for the key-value editor. */
export function useKeyValueEditor({ value, onChange, max }: UseKeyValueEditorOptions) {
  const [pairs, setPairs] = useKeyValueSync(value);

  const emitChange = useCallback(
    (next: typeof pairs) => {
      setPairs(next);
      onChange(toRecord(next));
    },
    [onChange, setPairs],
  );

  const { updatePair, removePair } = useKeyValueHandlers(pairs, emitChange);

  const addPair = useCallback(() => {
    if (max !== undefined && pairs.length >= max) return;
    setPairs([...pairs, { key: "", value: "" }]);
  }, [pairs, max, setPairs]);

  const handleUpdateKey = useCallback(
    (i: number) => (e: React.ChangeEvent<HTMLInputElement>) => updatePair(i, "key", e.target.value),
    [updatePair],
  );
  const handleUpdateValue = useCallback(
    (i: number) => (e: React.ChangeEvent<HTMLInputElement>) =>
      updatePair(i, "value", e.target.value),
    [updatePair],
  );
  const handleRemovePair = useCallback((i: number) => () => removePair(i), [removePair]);

  const atMax = max !== undefined && pairs.length >= max;
  return { pairs, addPair, handleUpdateKey, handleUpdateValue, handleRemovePair, atMax };
}
