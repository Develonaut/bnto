/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for group node parameters. */
export const groupParamsSchema = z.object({
  mode: z
    .enum(["sequential", "parallel"] as const)
    .optional()
    .default("sequential"),
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
