/** Convert a pair array back to a record, skipping pairs with empty keys. */

import type { KeyValuePair } from "./toPairs";

export function toRecord(pairs: KeyValuePair[]): Record<string, string> {
  const result: Record<string, string> = {};
  for (const { key, value } of pairs) {
    if (key.trim()) result[key.trim()] = value;
  }
  return result;
}
