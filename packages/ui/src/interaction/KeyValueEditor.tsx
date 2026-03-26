"use client";

import type { ComponentProps } from "react";

import { PlusIcon } from "../icons";
import { cn } from "../utils/cn";
import { Button } from "./Button";
import { KeyValueRow } from "./KeyValueRow";
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
      {kv.pairs.map((pair, i) => (
        <KeyValueRow
          key={i}
          pairKey={pair.key}
          pairValue={pair.value}
          onKeyChange={kv.handleUpdateKey(i)}
          onValueChange={kv.handleUpdateValue(i)}
          onRemove={kv.handleRemovePair(i)}
          keyPlaceholder={keyPlaceholder}
          valuePlaceholder={valuePlaceholder}
          disabled={disabled}
        />
      ))}
      {!kv.atMax && !disabled && (
        <Button type="button" variant="outline" onClick={kv.addPair} className="self-start">
          <PlusIcon />
          Add pair
        </Button>
      )}
    </div>
  );
}

export { KeyValueEditor };
export type { KeyValueEditorProps };
