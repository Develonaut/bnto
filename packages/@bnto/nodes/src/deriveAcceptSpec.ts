/**
 * Derives an AcceptSpec from a definition's input node.
 *
 * Extracts `accept`, `extensions`, and `label` from the input node's
 * parameters and returns them as an AcceptSpec. This is the single
 * source of truth — Recipe.accept is populated from here, not duplicated.
 */

import type { Definition } from "./definition";
import type { AcceptSpec } from "./recipe";
import { getInputNode } from "./getInputNode";

/** Derives an AcceptSpec from the input node in a definition. */
export function deriveAcceptSpec(definition: Definition): AcceptSpec | undefined {
  const inputNode = getInputNode(definition);
  if (!inputNode) return undefined;

  const params = inputNode.parameters;
  const mimeTypes = (params.accept as string[] | undefined) ?? [];
  const extensions = (params.extensions as string[] | undefined) ?? [];
  const label = (params.label as string | undefined) ?? "Any files";

  return { mimeTypes, extensions, label };
}
