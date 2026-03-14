type KeyValuePair = { key: string; value: string };

/** Convert record to ordered pair array for stable rendering. */
export function toPairs(record: Record<string, string>): KeyValuePair[] {
  return Object.entries(record).map(([key, value]) => ({ key, value }));
}

/** Convert pair array back to record, skipping pairs with empty keys. */
export function toRecord(pairs: KeyValuePair[]): Record<string, string> {
  const result: Record<string, string> = {};
  for (const { key, value } of pairs) {
    if (key.trim()) result[key.trim()] = value;
  }
  return result;
}

export type { KeyValuePair };
