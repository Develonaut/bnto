/**
 * EditorDebugApi — the shape of window.__bnto__.editor.
 *
 * Seven debug commands + state access + domain clients.
 * Keep this surface minimal — add methods as needed.
 */

import type { EditorState } from "../store/types";
import type {
  NodeClient,
  DefinitionClient,
  ExecutionClient,
  HistoryClient,
  PanelClient,
} from "../editorTypes";

interface EditorDebugApi {
  // --- State access ---

  /** Deep-cloned snapshot — safe to inspect without affecting the store. */
  getState(): EditorState;

  /** Direct store patch — merges partial state into the store. */
  setState(partial: Partial<EditorState>): void;

  /** Watch store changes. Returns an unsubscribe function. */
  subscribe(fn: (state: EditorState) => void): () => void;

  // --- Domain clients (same API as components) ---

  nodes: NodeClient;
  definition: DefinitionClient;
  execution: ExecutionClient;
  history: HistoryClient;
  panels: PanelClient;

  // --- Debug commands ---

  /** Auto-step through the full pipeline with a delay between steps (default 800ms). */
  run(delayMs?: number): Promise<void>;

  /** Advance execution by one step. Call repeatedly to walk through the pipeline. */
  step(): string;

  /** Force the editor into an error state with a message. */
  error(message: string): void;

  /** Force all nodes to completed and set phase to completed. */
  complete(): void;

  /** Data-only snapshot of state (functions stripped) for console inspection. */
  snapshot(): Record<string, unknown>;

  /** Reset execution state back to idle — keep recipe, ready to run again. */
  reset(): void;

  /** Wipe everything — blank recipe, fresh I/O nodes, all state cleared. */
  clear(): void;
}

export type { EditorDebugApi };
