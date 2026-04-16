/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for parallel node parameters. */
export const parallelParamsSchema = z.object({
  tasks: z.record(z.unknown()),
  maxWorkers: z.number().min(1).optional(),
  errorStrategy: z
    .enum(["failFast", "collectAll"] as const)
    .optional()
    .default("failFast"),
});

/** Inferred TypeScript type for parallel node parameters. */
export type ParallelParams = z.infer<typeof parallelParamsSchema>;

/** Full schema definition for the parallel node type. */
export const parallelNodeSchema: NodeSchema = {
  nodeType: "parallel",
  schemaVersion: 1,
  schema: parallelParamsSchema,
  params: {
    tasks: {
      label: "Tasks",
      description: "Array of task definitions to execute concurrently.",
      surfaceable: false,
    },
    maxWorkers: {
      label: "Max Workers",
      description: "Maximum number of concurrent workers. Defaults to the number of tasks.",
    },
    errorStrategy: {
      label: "Error Strategy",
      description: "How to handle task errors — fail immediately or collect all results.",
    },
  },
};
