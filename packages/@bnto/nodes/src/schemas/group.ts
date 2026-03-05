/**
 * Group node schema — parameters for the container node.
 *
 * Go source: engine/pkg/node/library/group/group.go
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Valid group execution modes. */
export const GROUP_MODES = ["sequential", "parallel"] as const;

/** Zod schema for group node parameters. */
export const groupParamsSchema = z.object({
  mode: z.enum(GROUP_MODES).optional().default("sequential"),
});

/** Inferred TypeScript type for group node parameters. */
export type GroupParams = z.infer<typeof groupParamsSchema>;

/** Full schema definition for the group node type. */
export const groupNodeSchema: NodeSchemaDefinition = {
  nodeType: "group",
  schemaVersion: 1,
  schema: groupParamsSchema,
  params: {
    mode: {
      label: "Mode",
      description:
        "How child nodes execute — sequentially (one after another) or in parallel (concurrently).",
    },
  },
};
