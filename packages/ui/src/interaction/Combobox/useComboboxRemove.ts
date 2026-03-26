"use client";

import { useCallback } from "react";

/** Handlers for removing items from a combobox selection. */
export function useComboboxRemove(value: string[], onChange: (values: string[]) => void) {
  const removeOption = useCallback(
    (optionValue: string) => onChange(value.filter((v) => v !== optionValue)),
    [value, onChange],
  );

  const handleRemoveMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const handleRemoveClick = useCallback(
    (e: React.MouseEvent<HTMLSpanElement>) => {
      e.stopPropagation();
      const optionValue = e.currentTarget.dataset.value;
      if (optionValue) removeOption(optionValue);
    },
    [removeOption],
  );

  const handleRemoveKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLSpanElement>) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        e.stopPropagation();
        const optionValue = e.currentTarget.dataset.value;
        if (optionValue) removeOption(optionValue);
      }
    },
    [removeOption],
  );

  return { handleRemoveMouseDown, handleRemoveClick, handleRemoveKeyDown };
}
