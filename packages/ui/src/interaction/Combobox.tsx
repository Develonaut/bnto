"use client";

import type { ComponentProps } from "react";

import { CheckIcon, ChevronsUpDownIcon } from "../icons";
import { cn } from "../utils/cn";
import { Popover } from "../overlay/Popover";
import { PopupTrigger } from "../overlay/PopupTrigger";
import { PopupContent } from "../overlay/PopupContent";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "./Command";
import { useCombobox } from "./useCombobox";
import { ComboboxBadges } from "./ComboboxBadges";

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
        <PopupTrigger
          open={cb.open}
          role="combobox"
          aria-expanded={cb.open}
          disabled={disabled}
          anchorClassName="w-full"
          className="w-full justify-between font-normal"
        >
          <div className="flex flex-1 flex-wrap items-center gap-1 overflow-hidden">
            <ComboboxBadges
              value={value}
              options={options}
              maxVisible={maxVisible}
              placeholder={placeholder}
              onRemoveMouseDown={cb.handleRemoveMouseDown}
              onRemoveClick={cb.handleRemoveClick}
              onRemoveKeyDown={cb.handleRemoveKeyDown}
            />
          </div>
          <ChevronsUpDownIcon className="size-4 shrink-0 opacity-50" />
        </PopupTrigger>
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

/* -- Dropdown list ------------------------------------------------ */

function ComboboxDropdown({
  options,
  value,
  max,
  searchPlaceholder,
  emptyText,
  onToggle,
}: {
  options: ComboboxOption[];
  value: string[];
  max?: number;
  searchPlaceholder: string;
  emptyText: string;
  onToggle: (v: string) => () => void;
}) {
  return (
    <PopupContent className="w-[var(--radix-popover-trigger-width)] p-0" align="start">
      <Command>
        <CommandInput placeholder={searchPlaceholder} />
        <CommandList>
          <CommandEmpty>{emptyText}</CommandEmpty>
          <CommandGroup>
            {options.map((option) => {
              const selected = value.includes(option.value);
              const atMax = max !== undefined && value.length >= max;
              return (
                <CommandItem
                  key={option.value}
                  value={option.value}
                  onSelect={onToggle(option.value)}
                  disabled={!selected && atMax}
                >
                  <CheckIcon className={cn("size-4", selected ? "opacity-100" : "opacity-0")} />
                  {option.label}
                </CommandItem>
              );
            })}
          </CommandGroup>
        </CommandList>
      </Command>
    </PopupContent>
  );
}

export { Combobox };
export type { ComboboxProps, ComboboxOption };
