"use client";

import { useState, useCallback, type ComponentProps } from "react";

import { CheckIcon, ChevronsUpDownIcon, XIcon } from "../icons";
import { cn } from "../utils/cn";
import { Badge } from "../typography/Badge";
import { PopupTriggerButton } from "./PopupTriggerButton";
import { Popover, PopoverAnchor, PopoverContent, PopoverTrigger } from "../overlay/Popover";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "./Command";

type ComboboxOption = {
  value: string;
  label: string;
};

type ComboboxProps = Omit<ComponentProps<"div">, "onChange"> & {
  /** Available options to select from. */
  options: ComboboxOption[];
  /** Currently selected values. */
  value: string[];
  /** Called when selection changes. */
  onChange: (values: string[]) => void;
  /** Placeholder when nothing is selected. */
  placeholder?: string;
  /** Placeholder for the search input. */
  searchPlaceholder?: string;
  /** Text shown when no options match the search. */
  emptyText?: string;
  /** Maximum number of selections. */
  max?: number;
  /** Max visible badges before showing "+N more". Defaults to 2. */
  maxVisible?: number;
  /** Disable the combobox. */
  disabled?: boolean;
};

function Combobox({
  options,
  value,
  onChange,
  placeholder = "Select…",
  searchPlaceholder = "Search…",
  emptyText = "No results found.",
  max,
  maxVisible = 2,
  disabled = false,
  className,
  ...props
}: ComboboxProps) {
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

  const removeOption = useCallback(
    (optionValue: string) => {
      onChange(value.filter((v) => v !== optionValue));
    },
    [value, onChange],
  );

  const handleToggleOption = useCallback(
    (optionValue: string) => () => toggleOption(optionValue),
    [toggleOption],
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

  return (
    <div data-slot="combobox" className={className} {...props}>
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverAnchor className="inline-flex w-full">
          <PopoverTrigger asChild>
            <PopupTriggerButton
              open={open}
              role="combobox"
              aria-expanded={open}
              disabled={disabled}
              className="w-full justify-between font-normal"
            >
              <div className="flex flex-1 flex-wrap items-center gap-1 overflow-hidden">
                {value.length > 0 ? (
                  <>
                    {value.slice(0, maxVisible).map((v) => {
                      const label = options.find((o) => o.value === v)?.label ?? v;
                      return (
                        <Badge key={v} variant="secondary" size="sm">
                          {label}
                          <span
                            role="button"
                            tabIndex={0}
                            data-value={v}
                            className="ml-0.5 cursor-pointer rounded-full outline-none focus-visible:ring-1 focus-visible:ring-ring"
                            onMouseDown={handleRemoveMouseDown}
                            onClick={handleRemoveClick}
                            onKeyDown={handleRemoveKeyDown}
                            aria-label={`Remove ${label}`}
                          >
                            <XIcon className="size-3 text-muted-foreground hover:text-foreground" />
                          </span>
                        </Badge>
                      );
                    })}
                    {value.length > maxVisible && (
                      <span className="text-xs text-muted-foreground">
                        +{value.length - maxVisible} more
                      </span>
                    )}
                  </>
                ) : (
                  <span className="text-muted-foreground">{placeholder}</span>
                )}
              </div>
              <ChevronsUpDownIcon className="size-4 shrink-0 opacity-50" />
            </PopupTriggerButton>
          </PopoverTrigger>
        </PopoverAnchor>
        <PopoverContent className="w-[var(--radix-popover-trigger-width)] p-0" align="start">
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
                      onSelect={handleToggleOption(option.value)}
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
        </PopoverContent>
      </Popover>
    </div>
  );
}

export { Combobox };
export type { ComboboxProps, ComboboxOption };
