/** Parallel node schema — parameters for concurrent task execution. */

import { z } from "zod";
import type { FieldConfigMap, NodeSchemaDefinition } from "./types";

/** Valid error handling strategies for parallel execution. */
export const ERROR_STRATEGIES = ["failFast", "collectAll"] as const;

/** Zod schema for parallel node parameters. */
export const parallelParamsSchema = z.object({
  tasks: z.array(z.record(z.unknown())),
  maxWorkers: z.number().min(1).optional(),
  errorStrategy: z.enum(ERROR_STRATEGIES).optional().default("failFast"),
});

/** Inferred TypeScript type for parallel node parameters. */
export type ParallelParams = z.infer<typeof parallelParamsSchema>;

/** Full schema definition for the parallel node type. */
export const parallelNodeSchema: NodeSchemaDefinition = {
  nodeType: "parallel",
  schemaVersion: 1,
  schema: parallelParamsSchema,
  params: {
    tasks: {
      label: "Tasks",
      description: "Array of task definitions to execute concurrently.",
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

/** UI presentation metadata for parallel node fields. */
export const parallelFields: FieldConfigMap = {
  tasks: { hidden: true },
  errorStrategy: {
    options: [
      { value: "failFast", label: "Fail Fast" },
      { value: "collectAll", label: "Collect All" },
    ],
  },
};
