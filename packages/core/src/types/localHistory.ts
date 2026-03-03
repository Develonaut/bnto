/** A local execution history entry stored in IndexedDB. */
export interface LocalHistoryEntry {
  id: string;
  slug: string;
  status: "completed" | "failed";
  /** Unix timestamp (ms) when the execution started. */
  timestamp: number;
  durationMs: number;
  inputFileCount: number;
  outputFileCount: number;
  error?: string;
}
