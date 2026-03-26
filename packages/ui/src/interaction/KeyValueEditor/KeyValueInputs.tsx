import type { ChangeEventHandler } from "react";

import { Input } from "../Input";

interface KeyValueInputsProps {
  pairKey: string;
  pairValue: string;
  onKeyChange: ChangeEventHandler<HTMLInputElement>;
  onValueChange: ChangeEventHandler<HTMLInputElement>;
  keyPlaceholder: string;
  valuePlaceholder: string;
  disabled: boolean;
}

export function KeyValueInputs({
  pairKey,
  pairValue,
  onKeyChange,
  onValueChange,
  keyPlaceholder,
  valuePlaceholder,
  disabled,
}: KeyValueInputsProps) {
  return (
    <>
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
    </>
  );
}
