/**
 * Derive accepted file types from a definition's input node.
 *
 * Reads the input node's parameters (accept, extensions, label, mimePrefix)
 * and returns them in the shape consumed by file validation and drop zones.
 * Falls back to wildcard when no input node is found.
 */

import { getInputNode } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";

interface AcceptedTypes {
  /** Value for the HTML <input accept="..."> attribute. */
  accept: string;
  /** Human-readable label shown in the drop zone. */
  label: string;
  /** MIME type prefix to validate against (e.g. "image/"). */
  mimePrefix?: string;
}

const WILDCARD: AcceptedTypes = { accept: "*/*", label: "files" };

/**
 * Derives accepted file types from a definition's input node.
 *
 * Returns the accept string, label, and optional MIME prefix. If no input
 * node is found or the input node has no accept constraint, returns wildcard.
 */
export function deriveAcceptedTypes(definition: Definition): AcceptedTypes {
  const inputNode = getInputNode(definition);
  if (!inputNode) return WILDCARD;

  const params = inputNode.parameters;
  const mimeTypes = (params.accept as string[] | undefined) ?? [];
  const extensions = (params.extensions as string[] | undefined) ?? [];
  const label = (params.label as string | undefined) ?? "files";

  if (mimeTypes.length === 0 && extensions.length === 0) {
    return { accept: "*/*", label };
  }

  // Wildcard MIME — accept anything
  if (mimeTypes.length === 1 && mimeTypes[0] === "*/*") {
    return { accept: "*/*", label };
  }

  // Derive mimePrefix from common MIME type prefix (e.g. all "image/*")
  const mimePrefix = deriveMimePrefix(mimeTypes);

  // When a mimePrefix is set, MIME types alone are sufficient for filtering.
  // Extensions are redundant and would bloat the accept string.
  const parts = mimePrefix ? mimeTypes : [...mimeTypes, ...extensions];
  return { accept: parts.join(","), label, mimePrefix };
}

/** If all MIME types share a common prefix (e.g. "image/"), return it. */
function deriveMimePrefix(mimeTypes: string[]): string | undefined {
  if (mimeTypes.length === 0) return undefined;

  const prefix = mimeTypes[0]!.split("/")[0]!;
  const allMatch = mimeTypes.every((t) => t.startsWith(`${prefix}/`));
  return allMatch ? `${prefix}/` : undefined;
}
