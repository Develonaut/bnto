/** Group node schema — parameters for the container node. */

import { z } from "zod";
import type { NodeParamFields, NodeSchema } from "./types";

/** Valid group execution modes. */
export const GROUP_MODES = ["sequential", "parallel"] as const;

/** Zod schema for group node parameters. */
export const groupParamsSchema = z.object({
  mode: z.enum(GROUP_MODES).optional().default("sequential"),
});

/** Inferred TypeScript type for group node parameters. */
export type GroupParams = z.infer<typeof groupParamsSchema>;

/** Full schema definition for the group node type. */
export const groupNodeSchema: NodeSchema = {
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

/** UI presentation metadata for group node fields. */
export const groupFields: NodeParamFields = {};
