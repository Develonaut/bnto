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

const INPUT_MODE_LABELS: Record<string, string> = {
  "file-upload": "File Upload",
  text: "Text Input",
  url: "URL",
};

const OUTPUT_MODE_LABELS: Record<string, string> = {
  download: "Download",
  display: "Display",
  preview: "Preview",
};

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
  if (nodeType === "input") {
    return INPUT_MODE_LABELS[params?.mode as string] ?? "Input";
  }
  if (nodeType === "output") {
    return OUTPUT_MODE_LABELS[params?.mode as string] ?? "Output";
  }

  const info = NODE_TYPE_INFO[nodeType];

  // Pre-composed sub-recipes (containers with displayName) show "Recipe".
  // Bare control flow nodes show their own label ("Loop", "Group", "Parallel").
  if (info.category === "control") {
    if (metadata?.customData?.displayName) return "Recipe";
    return info.label;
  }

  return getCategoryInfo(info.category)?.label ?? nodeType;
}
