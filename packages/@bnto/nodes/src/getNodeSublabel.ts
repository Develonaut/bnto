/**
 * getNodeSublabel — human-readable sublabel for a node.
 *
 * I/O nodes derive the sublabel from their configured mode param.
 * Control flow nodes use their own label (e.g. "Loop", not "Control Flow").
 * All other nodes use the category label ("Image", "Spreadsheet", etc.).
 *
 * Lives in @bnto/nodes so sublabels are defined alongside node metadata —
 * consumers call getNodeSublabel(type, params) without hardcoded maps.
 */

import type { NodeTypeName } from "./nodeTypes";
import { NODE_TYPE_INFO } from "./nodeTypes";
import { getCategoryInfo } from "./categories";
import { INPUT_MODES, OUTPUT_MODES } from "./ioModes";

const IO_LABEL_LOOKUP: Record<string, { modes: Record<string, string>; fallback: string }> = {
  input: { modes: toLabelMap(INPUT_MODES), fallback: "Input" },
  output: { modes: toLabelMap(OUTPUT_MODES), fallback: "Output" },
};

function toLabelMap(modes: Record<string, { label: string }>): Record<string, string> {
  const map: Record<string, string> = {};
  for (const [key, val] of Object.entries(modes)) map[key] = val.label;
  return map;
}

function resolveControlSublabel(
  label: string,
  metadata?: { customData?: Record<string, string> },
): string {
  if (metadata?.customData?.displayName) return "Recipe";
  return label;
}

/**
 * Returns the sublabel for a node given its type and params.
 *
 * For I/O nodes, the sublabel reflects the configured mode.
 * For control flow nodes with a displayName, the sublabel is "Recipe"
 * (it's a pre-composed sub-recipe, not a bare group).
 * For bare control flow nodes, the sublabel is the node's own label.
 * For everything else, the sublabel is the category label.
 */
export function getNodeSublabel(
  nodeType: NodeTypeName,
  params?: Record<string, unknown>,
  metadata?: { customData?: Record<string, string> },
): string {
  const io = IO_LABEL_LOOKUP[nodeType];
  if (io) {
    const mode = params?.mode as string | undefined;
    return (mode && io.modes[mode]) || io.fallback;
  }

  const info = NODE_TYPE_INFO[nodeType];
  if (!info) return nodeType;

  if (info.category === "control") return resolveControlSublabel(info.label, metadata);
  return getCategoryInfo(info.category)?.label ?? nodeType;
}
