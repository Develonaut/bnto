"use client";

import { PopupContent } from "../../overlay/PopupContent";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandList } from "../Command";
import { ComboboxOptionItem } from "./ComboboxOptionItem";

import type { ComboboxOption } from "./ComboboxRoot";

interface ComboboxDropdownProps {
  options: ComboboxOption[];
  value: string[];
  max?: number;
  searchPlaceholder: string;
  emptyText: string;
  onToggle: (v: string) => () => void;
}

export function ComboboxDropdown({
  options,
  value,
  max,
  searchPlaceholder,
  emptyText,
  onToggle,
}: ComboboxDropdownProps) {
  const atMax = max !== undefined && value.length >= max;
  return (
    <PopupContent className="w-[var(--radix-popover-trigger-width)] p-0" align="start">
      <Command>
        <CommandInput placeholder={searchPlaceholder} />
        <CommandList>
          <CommandEmpty>{emptyText}</CommandEmpty>
          <CommandGroup>
            {options.map((o) => (
              <ComboboxOptionItem
                key={o.value}
                value={o.value}
                label={o.label}
                selected={value.includes(o.value)}
                disabled={!value.includes(o.value) && atMax}
                onSelect={onToggle(o.value)}
              />
            ))}
          </CommandGroup>
        </CommandList>
      </Command>
    </PopupContent>
  );
}
