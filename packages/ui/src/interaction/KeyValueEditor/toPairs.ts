/** Convert a record to an ordered pair array for stable rendering. */

export type KeyValuePair = { key: string; value: string };

export function toPairs(record: Record<string, string>): KeyValuePair[] {
  return Object.entries(record).map(([key, value]) => ({ key, value }));
}
