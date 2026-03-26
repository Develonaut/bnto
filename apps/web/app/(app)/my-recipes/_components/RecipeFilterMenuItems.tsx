"use client";

import { useCallback } from "react";
import { CheckIcon, MenuItem } from "@bnto/ui";

interface FilterMenuItemsProps<T extends string> {
  options: { value: T; label: string }[];
  selected: T;
  onSelect: (value: T) => void;
  testIdPrefix: string;
}

function FilterMenuItem<T extends string>({
  opt,
  selected,
  onSelect,
  testIdPrefix,
}: {
  opt: { value: T; label: string };
  selected: T;
  onSelect: (value: T) => void;
  testIdPrefix: string;
}) {
  const handleClick = useCallback(() => onSelect(opt.value), [onSelect, opt.value]);

  return (
    <MenuItem onClick={handleClick} data-testid={`${testIdPrefix}-${opt.value}`}>
      {selected === opt.value ? (
        <CheckIcon className="size-4 text-primary" />
      ) : (
        <span className="size-4" />
      )}
      {opt.label}
    </MenuItem>
  );
}

export function FilterMenuItems<T extends string>({
  options,
  selected,
  onSelect,
  testIdPrefix,
}: FilterMenuItemsProps<T>) {
  return (
    <>
      {options.map((opt) => (
        <FilterMenuItem
          key={opt.value}
          opt={opt}
          selected={selected}
          onSelect={onSelect}
          testIdPrefix={testIdPrefix}
        />
      ))}
    </>
  );
}
