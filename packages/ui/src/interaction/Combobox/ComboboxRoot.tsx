"use client";

import type { ComponentProps } from "react";

import { Popover } from "../../overlay/Popover";
import { useCombobox } from "./useCombobox";
import { ComboboxTrigger } from "./ComboboxTrigger";
import { ComboboxDropdown } from "./ComboboxDropdown";

type ComboboxOption = { value: string; label: string };

type ComboboxProps = Omit<ComponentProps<"div">, "onChange"> & {
  options: ComboboxOption[];
  value: string[];
  onChange: (values: string[]) => void;
  placeholder?: string;
  searchPlaceholder?: string;
  emptyText?: string;
  max?: number;
  maxVisible?: number;
  disabled?: boolean;
};

function Combobox({
  options,
  value,
  onChange,
  placeholder = "Select\u2026",
  searchPlaceholder = "Search\u2026",
  emptyText = "No results found.",
  max,
  maxVisible = 2,
  disabled = false,
  className,
  ...props
}: ComboboxProps) {
  const cb = useCombobox({ value, onChange, max });
  return (
    <div data-slot="combobox" className={className} {...props}>
      <Popover open={cb.open} onOpenChange={cb.setOpen}>
        <ComboboxTrigger
          open={cb.open}
          disabled={disabled}
          value={value}
          options={options}
          maxVisible={maxVisible}
          placeholder={placeholder}
          onRemoveMouseDown={cb.handleRemoveMouseDown}
          onRemoveClick={cb.handleRemoveClick}
          onRemoveKeyDown={cb.handleRemoveKeyDown}
        />
        <ComboboxDropdown
          options={options}
          value={value}
          max={max}
          searchPlaceholder={searchPlaceholder}
          emptyText={emptyText}
          onToggle={cb.handleToggleOption}
        />
      </Popover>
    </div>
  );
}

export { Combobox };
export type { ComboboxProps, ComboboxOption };
