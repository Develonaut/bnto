/**
 * createDebouncedSave — debounce factory for auto-save.
 *
 * Thin wrapper over the generalized `debounce` utility,
 * preserving the domain-specific type alias.
 */

import { debounce } from "./debounce";
import type { Debounced } from "./debounce";

type DebouncedSave = Debounced;

function createDebouncedSave(delayMs: number): DebouncedSave {
  return debounce(delayMs);
}

export { createDebouncedSave };
export type { DebouncedSave };
