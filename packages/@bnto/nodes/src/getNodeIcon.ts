/**
 * getNodeIcon — single source of truth for what icon a node shows.
 *
 * Processing nodes delegate to the static `icon` field in NODE_TYPE_INFO.
 * I/O nodes derive the icon from params (input mode, output mode).
 *
 * This lives in @bnto/nodes because the icon is metadata about a node
 * type — the editor just consumes getNodeIcon(nodeType, params) and
 * gets back a Lucide icon name string.
 */

import type { NodeTypeName } from "./nodeTypes";
import { NODE_TYPE_INFO } from "./nodeTypes";
import { INPUT_MODES, OUTPUT_MODES } from "./ioModes";

const IO_ICON_LOOKUP: Record<string, { modes: Record<string, string>; fallback: string }> = {
  input: { modes: toIconMap(INPUT_MODES), fallback: "file-up" },
  output: { modes: toIconMap(OUTPUT_MODES), fallback: "download" },
};

function toIconMap(modes: Record<string, { icon: string }>): Record<string, string> {
  const map: Record<string, string> = {};
  for (const [key, val] of Object.entries(modes)) map[key] = val.icon;
  return map;
}

/**
 * Returns the Lucide icon name for a node given its type and params.
 *
 * For I/O nodes, the icon reflects the configured mode (file-upload,
 * text, url, write, overwrite, message, none). For processing nodes,
 * the icon is the static value from NODE_TYPE_INFO.
 */
export function getNodeIcon(nodeType: NodeTypeName, params?: Record<string, unknown>): string {
  const io = IO_ICON_LOOKUP[nodeType];
  if (io) {
    const mode = params?.mode as string | undefined;
    return (mode && io.modes[mode]) || io.fallback;
  }
  return NODE_TYPE_INFO[nodeType]?.icon ?? "box";
}
