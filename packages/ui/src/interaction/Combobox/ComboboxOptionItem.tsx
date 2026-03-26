"use client";

import { CheckIcon } from "../../icons";
import { cn } from "../../utils/cn";
import { CommandItem } from "../Command";

interface ComboboxOptionItemProps {
  value: string;
  label: string;
  selected: boolean;
  disabled: boolean;
  onSelect: () => void;
}

export function ComboboxOptionItem({
  value,
  label,
  selected,
  disabled,
  onSelect,
}: ComboboxOptionItemProps) {
  return (
    <CommandItem key={value} value={value} onSelect={onSelect} disabled={disabled}>
      <CheckIcon className={cn("size-4", selected ? "opacity-100" : "opacity-0")} />
      {label}
    </CommandItem>
  );
}
