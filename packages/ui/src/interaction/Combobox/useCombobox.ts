"use client";

import { useState, useCallback } from "react";

import { useComboboxRemove } from "./useComboboxRemove";

interface UseComboboxOptions {
  value: string[];
  onChange: (values: string[]) => void;
  max?: number;
}

/** State and handlers for multi-select combobox. */
export function useCombobox({ value, onChange, max }: UseComboboxOptions) {
  const [open, setOpen] = useState(false);

  const toggleOption = useCallback(
    (optionValue: string) => {
      if (value.includes(optionValue)) {
        onChange(value.filter((v) => v !== optionValue));
      } else {
        if (max !== undefined && value.length >= max) return;
        onChange([...value, optionValue]);
      }
    },
    [value, onChange, max],
  );

  const handleToggleOption = useCallback(
    (optionValue: string) => () => toggleOption(optionValue),
    [toggleOption],
  );

  const remove = useComboboxRemove(value, onChange);

  return { open, setOpen, handleToggleOption, ...remove };
}
