import { Input, Label } from "@bnto/ui";
import type { ChangeEvent } from "react";

interface PrefixInputProps {
  id: string;
  value: string;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
  placeholder?: string;
}

/** Text prefix input field with label. */
export function PrefixInput({ id, value, onChange, placeholder }: PrefixInputProps) {
  return (
    <div className="flex shrink-0 flex-col gap-1">
      <Label htmlFor={id} className="text-muted-foreground text-xs">
        Prefix
      </Label>
      <Input
        id={id}
        type="text"
        wrapperClassName="w-28"
        value={value}
        onChange={onChange}
        placeholder={placeholder}
      />
    </div>
  );
}
