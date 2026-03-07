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
import type { InputParams } from "./schemas/input";
import type { OutputParams } from "./schemas/output";
import { NODE_TYPE_INFO } from "./nodeTypes";

const INPUT_MODE_ICONS: Record<InputParams["mode"], string> = {
  "file-upload": "file-up",
  text: "text-cursor-input",
  url: "link",
};

const OUTPUT_MODE_ICONS: Record<OutputParams["mode"], string> = {
  download: "download",
  display: "monitor",
  preview: "eye",
};

/**
 * Returns the Lucide icon name for a node given its type and params.
 *
 * For I/O nodes, the icon reflects the configured mode (file-upload,
 * text, url, download, display, preview). For processing nodes, the
 * icon is the static value from NODE_TYPE_INFO.
 */
export function getNodeIcon(nodeType: NodeTypeName, params?: Record<string, unknown>): string {
  if (nodeType === "input" && params?.mode) {
    return INPUT_MODE_ICONS[params.mode as InputParams["mode"]] ?? "file-up";
  }
  if (nodeType === "output" && params?.mode) {
    return OUTPUT_MODE_ICONS[params.mode as OutputParams["mode"]] ?? "download";
  }
  return NODE_TYPE_INFO[nodeType].icon;
}
