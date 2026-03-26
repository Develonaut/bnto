"use client";

import { useCallback } from "react";

import type { KeyValuePair } from "./toPairs";
import { toRecord } from "./toRecord";
import { useKeyValueSync } from "./useKeyValueSync";

interface UseKeyValueEditorOptions {
  value: Record<string, string>;
  onChange: (record: Record<string, string>) => void;
  max?: number;
}

/** State and handlers for the key-value editor. */
export function useKeyValueEditor({ value, onChange, max }: UseKeyValueEditorOptions) {
  const [pairs, setPairs] = useKeyValueSync(value);

  const emitChange = useCallback(
    (next: KeyValuePair[]) => {
      setPairs(next);
      onChange(toRecord(next));
    },
    [onChange, setPairs],
  );

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
  const addPair = useCallback(() => {
    if (max !== undefined && pairs.length >= max) return;
    setPairs([...pairs, { key: "", value: "" }]);
  }, [pairs, max, setPairs]);

  const handleUpdateKey = useCallback(
    (index: number) => (e: React.ChangeEvent<HTMLInputElement>) =>
      updatePair(index, "key", e.target.value),
    [updatePair],
  );
  const handleUpdateValue = useCallback(
    (index: number) => (e: React.ChangeEvent<HTMLInputElement>) =>
      updatePair(index, "value", e.target.value),
    [updatePair],
  );
  const handleRemovePair = useCallback((index: number) => () => removePair(index), [removePair]);

  const atMax = max !== undefined && pairs.length >= max;
  return { pairs, addPair, handleUpdateKey, handleUpdateValue, handleRemovePair, atMax };
}
