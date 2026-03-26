"use client";

import type { ChangeEventHandler, MouseEventHandler } from "react";

import { XIcon } from "../../icons";
import { Button } from "../Button";
import { KeyValueInputs } from "./KeyValueInputs";

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
      <KeyValueInputs
        pairKey={pairKey}
        pairValue={pairValue}
        onKeyChange={onKeyChange}
        onValueChange={onValueChange}
        keyPlaceholder={keyPlaceholder}
        valuePlaceholder={valuePlaceholder}
        disabled={disabled}
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
