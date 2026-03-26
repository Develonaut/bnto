import { Input, Label } from "@bnto/ui";
import type { ChangeEvent } from "react";

interface WidthInputProps {
  id: string;
  value: number;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
}

/** Shared width-in-pixels input used across image config panels. */
export function WidthInput({ id, value, onChange }: WidthInputProps) {
  return (
    <div className="flex shrink-0 flex-col gap-1">
      <Label htmlFor={id} className="text-muted-foreground text-xs">
        Width (px)
      </Label>
      <Input
        id={id}
        type="number"
        min={1}
        max={10000}
        value={value}
        wrapperClassName="w-24"
        onChange={onChange}
      />
    </div>
  );
}
