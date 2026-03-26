"use client";

import type { MouseEventHandler, KeyboardEventHandler } from "react";

import { ComboboxBadge } from "./ComboboxBadge";

interface ComboboxBadgesProps {
  value: string[];
  options: { value: string; label: string }[];
  maxVisible: number;
  placeholder: string;
  onRemoveMouseDown: MouseEventHandler;
  onRemoveClick: MouseEventHandler<HTMLSpanElement>;
  onRemoveKeyDown: KeyboardEventHandler<HTMLSpanElement>;
}

function findLabel(options: ComboboxBadgesProps["options"], v: string) {
  return options.find((o) => o.value === v)?.label ?? v;
}

export function ComboboxBadges({
  value,
  options,
  maxVisible,
  placeholder,
  onRemoveMouseDown,
  onRemoveClick,
  onRemoveKeyDown,
}: ComboboxBadgesProps) {
  if (value.length === 0) {
    return <span className="text-muted-foreground">{placeholder}</span>;
  }

  return (
    <>
      {value.slice(0, maxVisible).map((v) => (
        <ComboboxBadge
          key={v}
          value={v}
          label={findLabel(options, v)}
          onMouseDown={onRemoveMouseDown}
          onClick={onRemoveClick}
          onKeyDown={onRemoveKeyDown}
        />
      ))}
      {value.length > maxVisible && (
        <span className="text-xs text-muted-foreground">+{value.length - maxVisible} more</span>
      )}
    </>
  );
}
