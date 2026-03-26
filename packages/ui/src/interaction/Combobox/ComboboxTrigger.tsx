"use client";

import type { MouseEventHandler, KeyboardEventHandler } from "react";

import { ChevronsUpDownIcon } from "../../icons";
import { PopupTrigger } from "../../overlay/PopupTrigger";
import { ComboboxBadges } from "./ComboboxBadges";

interface ComboboxTriggerProps {
  open: boolean;
  disabled: boolean;
  value: string[];
  options: { value: string; label: string }[];
  maxVisible: number;
  placeholder: string;
  onRemoveMouseDown: MouseEventHandler;
  onRemoveClick: MouseEventHandler<HTMLSpanElement>;
  onRemoveKeyDown: KeyboardEventHandler<HTMLSpanElement>;
}

export function ComboboxTrigger(p: ComboboxTriggerProps) {
  return (
    <PopupTrigger
      open={p.open}
      role="combobox"
      aria-expanded={p.open}
      disabled={p.disabled}
      anchorClassName="w-full"
      className="w-full justify-between font-normal"
    >
      <div className="flex flex-1 flex-wrap items-center gap-1 overflow-hidden">
        <ComboboxBadges
          value={p.value}
          options={p.options}
          maxVisible={p.maxVisible}
          placeholder={p.placeholder}
          onRemoveMouseDown={p.onRemoveMouseDown}
          onRemoveClick={p.onRemoveClick}
          onRemoveKeyDown={p.onRemoveKeyDown}
        />
      </div>
      <ChevronsUpDownIcon className="size-4 shrink-0 opacity-50" />
    </PopupTrigger>
  );
}
