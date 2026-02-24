/**
 * Group node schema — parameters for the container node.
 *
 * Go source: engine/pkg/node/library/group/group.go
 */

import type { NodeSchema } from "./types";

/** Valid group execution modes. */
export const GROUP_MODES = ["sequential", "parallel"] as const;

export const groupSchema: NodeSchema = {
  nodeType: "group",
  parameters: [
    {
      name: "mode",
      type: "enum",
      required: false,
      label: "Mode",
      description:
        "How child nodes execute — sequentially (one after another) or in parallel (concurrently).",
      default: "sequential",
      enumValues: GROUP_MODES,
    },
  ],
} as const;
