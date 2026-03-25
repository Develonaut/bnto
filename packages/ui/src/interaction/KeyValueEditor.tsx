"use client";

import { useState, useCallback, useEffect, type ComponentProps } from "react";

import { PlusIcon, XIcon } from "../icons";
import { cn } from "../utils/cn";
import { Button } from "./Button";
import { Input } from "./Input";
import { toPairs, type KeyValuePair } from "./toPairs";
import { toRecord } from "./toRecord";

type KeyValueEditorProps = Omit<ComponentProps<"div">, "onChange"> & {
  /** Current key-value pairs as a record. */
  value: Record<string, string>;
  /** Called when pairs change (only includes pairs with non-empty keys). */
  onChange: (record: Record<string, string>) => void;
  /** Placeholder for key inputs. */
  keyPlaceholder?: string;
  /** Placeholder for value inputs. */
  valuePlaceholder?: string;
  /** Maximum number of pairs. Defaults to unlimited. */
  max?: number;
  /** Disable editing. */
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
  // Internal pairs array tracks empty-key rows that haven't been committed yet.
  const [pairs, setPairs] = useState<KeyValuePair[]>(() => toPairs(value));

  // Sync internal state when external value changes (e.g. undo/redo).
  useEffect(() => {
    const incoming = toPairs(value);
    // Only reset if the non-empty pairs actually differ from external value.
    const currentRecord = toRecord(pairs);
    const isSame =
      Object.keys(currentRecord).length === Object.keys(value).length &&
      Object.entries(currentRecord).every(([k, v]) => value[k] === v);
    if (!isSame) {
      setPairs(incoming);
    }
    // Intentionally only react to external `value` changes.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [value]);

  const emitChange = useCallback(
    (next: KeyValuePair[]) => {
      setPairs(next);
      onChange(toRecord(next));
    },
    [onChange],
  );

  const updatePair = useCallback(
    (index: number, field: "key" | "value", text: string) => {
      const next = [...pairs];
      next[index] = { ...next[index], [field]: text };
      emitChange(next);
    },
    [pairs, emitChange],
  );

  const removePair = useCallback(
    (index: number) => {
      emitChange(pairs.filter((_, i) => i !== index));
    },
    [pairs, emitChange],
  );

  const addPair = useCallback(() => {
    if (max !== undefined && pairs.length >= max) return;
    const next = [...pairs, { key: "", value: "" }];
    setPairs(next);
    // Don't emit — empty key won't appear in record anyway.
    // onChange fires when the user fills in the key.
  }, [pairs, max]);

  const handleUpdateKey = useCallback(
    (index: number) => (e: React.ChangeEvent<HTMLInputElement>) =>
      updatePair(index, "key", e.target.value),
    [updatePair],
  );

  const handleUpdateValue = useCallback(
    (index: number) => (e: React.ChangeEvent<HTMLInputElement>) =>
      updatePair(index, "value", e.target.value),
    [updatePair],
  );

  const handleRemovePair = useCallback((index: number) => () => removePair(index), [removePair]);

  const atMax = max !== undefined && pairs.length >= max;

  return (
    <div
      data-slot="key-value-editor"
      data-disabled={disabled || undefined}
      className={cn("flex flex-col gap-2", className)}
      {...props}
    >
      {pairs.map((pair, i) => (
        <div key={i} className="flex items-center gap-2">
          <Input
            value={pair.key}
            onChange={handleUpdateKey(i)}
            placeholder={keyPlaceholder}
            disabled={disabled}
            wrapperClassName="flex-1"
          />
          <span className="text-muted-foreground text-xs shrink-0">→</span>
          <Input
            value={pair.value}
            onChange={handleUpdateValue(i)}
            placeholder={valuePlaceholder}
            disabled={disabled}
            wrapperClassName="flex-1"
          />
          {!disabled && (
            <Button
              type="button"
              variant="outline"
              icon={<XIcon />}
              onClick={handleRemovePair(i)}
              aria-label={`Remove ${pair.key || "row"}`}
            />
          )}
        </div>
      ))}
      {!atMax && !disabled && (
        <Button type="button" variant="outline" onClick={addPair} className="self-start">
          <PlusIcon />
          Add pair
        </Button>
      )}
    </div>
  );
}

export { KeyValueEditor };
export type { KeyValueEditorProps };
