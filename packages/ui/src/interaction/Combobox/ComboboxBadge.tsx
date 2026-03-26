"use client";

import type { MouseEventHandler, KeyboardEventHandler } from "react";

import { XIcon } from "../../icons";
import { Badge } from "../../typography/Badge";

interface ComboboxBadgeProps {
  value: string;
  label: string;
  onMouseDown: MouseEventHandler;
  onClick: MouseEventHandler<HTMLSpanElement>;
  onKeyDown: KeyboardEventHandler<HTMLSpanElement>;
}

export function ComboboxBadge({
  value,
  label,
  onMouseDown,
  onClick,
  onKeyDown,
}: ComboboxBadgeProps) {
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
