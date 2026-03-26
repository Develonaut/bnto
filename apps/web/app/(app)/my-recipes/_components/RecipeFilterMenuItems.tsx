"use client";

import { CheckIcon, MenuItem } from "@bnto/ui";

interface FilterMenuItemsProps<T extends string> {
  options: { value: T; label: string }[];
  selected: T;
  onSelect: (value: T) => void;
  testIdPrefix: string;
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
        <MenuItem
          key={opt.value}
          onClick={() => onSelect(opt.value)}
          data-testid={`${testIdPrefix}-${opt.value}`}
        >
          {selected === opt.value ? (
            <CheckIcon className="size-4 text-primary" />
          ) : (
            <span className="size-4" />
          )}
          {opt.label}
        </MenuItem>
      ))}
    </>
  );
}
