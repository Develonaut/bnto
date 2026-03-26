"use client";

import { KeyValueRow } from "./KeyValueRow";
import type { useKeyValueEditor } from "./useKeyValueEditor";

interface KeyValueRowsProps {
  kv: ReturnType<typeof useKeyValueEditor>;
  keyPlaceholder: string;
  valuePlaceholder: string;
  disabled: boolean;
}

export function KeyValueRows({
  kv,
  keyPlaceholder,
  valuePlaceholder,
  disabled,
}: KeyValueRowsProps) {
  return (
    <>
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
    </>
  );
}
