"use client";

import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";
import { KeyValueRows } from "./KeyValueRows";
import { KeyValueAddButton } from "./KeyValueAddButton";
import { useKeyValueEditor } from "./useKeyValueEditor";

type KeyValueEditorProps = Omit<ComponentProps<"div">, "onChange"> & {
  value: Record<string, string>;
  onChange: (record: Record<string, string>) => void;
  keyPlaceholder?: string;
  valuePlaceholder?: string;
  max?: number;
  disabled?: boolean;
};

function KeyValueEditor({
  value,
  onChange,
  keyPlaceholder = "Key",
  valuePlaceholder = "Value",
  max,
  disabled = false,
  className,
  ...props
}: KeyValueEditorProps) {
  const kv = useKeyValueEditor({ value, onChange, max });

  return (
    <div
      data-slot="key-value-editor"
      data-disabled={disabled || undefined}
      className={cn("flex flex-col gap-2", className)}
      {...props}
    >
      <KeyValueRows
        kv={kv}
        keyPlaceholder={keyPlaceholder}
        valuePlaceholder={valuePlaceholder}
        disabled={disabled}
      />
      {!kv.atMax && !disabled && <KeyValueAddButton onClick={kv.addPair} />}
    </div>
  );
}

export { KeyValueEditor };
export type { KeyValueEditorProps };
