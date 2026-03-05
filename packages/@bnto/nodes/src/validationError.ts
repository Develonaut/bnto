/**
 * A single validation error with location and message.
 *
 * Extracted to its own file to prevent circular dependency between
 * validate.ts and validateTypeSpecific.ts.
 */

/** A single validation error with location and message. */
export interface ValidationError {
  /** The node ID where the error occurred. */
  nodeId: string;

  /** The field that failed validation (e.g., "type", "url", "mode"). */
  field: string;

  /** Human-readable error message. */
  message: string;
}
