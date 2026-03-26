"use client";

import type { MouseEventHandler, KeyboardEventHandler } from "react";

import { XIcon } from "../icons";
import { Badge } from "../typography/Badge";

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

function ComboboxBadge({
  value,
  label,
  onMouseDown,
  onClick,
  onKeyDown,
}: {
  value: string;
  label: string;
  onMouseDown: MouseEventHandler;
  onClick: MouseEventHandler<HTMLSpanElement>;
  onKeyDown: KeyboardEventHandler<HTMLSpanElement>;
}) {
  return (
    <Badge variant="secondary" size="sm">
      {label}
      <span
        role="button"
        tabIndex={0}
        data-value={value}
        className="ml-0.5 cursor-pointer rounded-full focus-ring focus-visible:outline-offset-0"
        onMouseDown={onMouseDown}
        onClick={onClick}
        onKeyDown={onKeyDown}
        aria-label={`Remove ${label}`}
      >
        <XIcon className="size-3 text-muted-foreground hover:text-foreground" />
      </span>
    </Badge>
  );
}
