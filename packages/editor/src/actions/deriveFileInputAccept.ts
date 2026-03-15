/**
 * deriveFileInputAccept — pure function that derives the HTML file input
 * `accept` attribute from the editor store's configs map.
 *
 * Reads the Input node's `accept` (MIME types) and `extensions` (file
 * extensions) parameters, then combines them into a comma-separated
 * string suitable for `<input type="file" accept="...">`.
 *
 * Returns undefined when no constraints are set (accept all files).
 */

import type { NodeConfigs } from "../adapters/types";

/**
 * Derives the `accept` string for the file input from the editor's
 * configs map. Finds the Input node, reads its accept + extensions
 * params, and merges them into a comma-separated accept string.
 *
 * Returns undefined when no constraints exist (wildcard).
 */
function deriveFileInputAccept(configs: NodeConfigs): string | undefined {
  const inputConfig = Object.values(configs).find((c) => c.nodeType === "input");
  if (!inputConfig) return undefined;

  const params = inputConfig.parameters;
  const mimeTypes = Array.isArray(params.accept) ? (params.accept as string[]) : [];
  const extensions = Array.isArray(params.extensions) ? (params.extensions as string[]) : [];

  if (mimeTypes.length === 0 && extensions.length === 0) return undefined;

  // Wildcard MIME — accept anything
  if (mimeTypes.length === 1 && mimeTypes[0] === "*/*") return undefined;

  // Combine MIME types and extensions into a single accept string.
  // The HTML accept attribute handles both formats natively.
  const parts = [...mimeTypes, ...extensions];
  return parts.join(",");
}

export { deriveFileInputAccept };
