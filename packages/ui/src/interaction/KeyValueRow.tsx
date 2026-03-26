"use client";

import type { ChangeEventHandler, MouseEventHandler } from "react";

import { XIcon } from "../icons";
import { Button } from "./Button";
import { Input } from "./Input";

interface KeyValueRowProps {
  pairKey: string;
  pairValue: string;
  onKeyChange: ChangeEventHandler<HTMLInputElement>;
  onValueChange: ChangeEventHandler<HTMLInputElement>;
  onRemove: MouseEventHandler;
  keyPlaceholder: string;
  valuePlaceholder: string;
  disabled: boolean;
}

export function KeyValueRow({
  pairKey,
  pairValue,
  onKeyChange,
  onValueChange,
  onRemove,
  keyPlaceholder,
  valuePlaceholder,
  disabled,
}: KeyValueRowProps) {
  return (
    <div className="flex items-center gap-2">
      <Input
        value={pairKey}
        onChange={onKeyChange}
        placeholder={keyPlaceholder}
        disabled={disabled}
        wrapperClassName="flex-1"
      />
      <span className="text-muted-foreground text-xs shrink-0">{"\u2192"}</span>
      <Input
        value={pairValue}
        onChange={onValueChange}
        placeholder={valuePlaceholder}
        disabled={disabled}
        wrapperClassName="flex-1"
      />
      {!disabled && (
        <Button
          type="button"
          variant="outline"
          icon={<XIcon />}
          onClick={onRemove}
          aria-label={`Remove ${pairKey || "row"}`}
        />
      )}
    </div>
  );
}
