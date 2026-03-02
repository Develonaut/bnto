/**
 * Result type for definition mutation operations.
 *
 * Every mutation (addNode, removeNode, updateNodeParams, etc.) returns
 * the updated definition AND any validation errors. The caller always
 * gets a definition back (best-effort result), and the errors tell you
 * what's wrong with it.
 *
 * This is the "apply what you can, report what failed" pattern —
 * the editor store sets both `definition` and `validationErrors` from
 * a single return value.
 */

import type { ValidationError } from "./validate";
import type { Definition } from "./definition";

/** Result of a definition mutation — always returns the definition + any errors. */
export interface DefinitionResult {
  /** The definition after the operation (always returned, may have validation errors). */
  definition: Definition;

  /** Validation errors found after applying the operation. Empty array = valid. */
  errors: ValidationError[];
}

/** Check if a mutation result has no validation errors. */
export function isValid(result: DefinitionResult): boolean {
  return result.errors.length === 0;
}
