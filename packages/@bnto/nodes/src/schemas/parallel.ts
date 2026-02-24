/**
 * Parallel node schema — parameters for concurrent task execution.
 *
 * Go source: engine/pkg/node/library/parallel/parallel.go
 */

import type { NodeSchema } from "./types";

/** Valid error handling strategies for parallel execution. */
export const ERROR_STRATEGIES = ["failFast", "collectAll"] as const;

export const parallelSchema: NodeSchema = {
  nodeType: "parallel",
  parameters: [
    {
      name: "tasks",
      type: "array",
      required: true,
      label: "Tasks",
      description: "Array of task definitions to execute concurrently.",
    },
    {
      name: "maxWorkers",
      type: "number",
      required: false,
      label: "Max Workers",
      description:
        "Maximum number of concurrent workers. Defaults to the number of tasks.",
      min: 1,
    },
    {
      name: "errorStrategy",
      type: "enum",
      required: false,
      label: "Error Strategy",
      description:
        "How to handle task errors — fail immediately or collect all results.",
      default: "failFast",
      enumValues: ERROR_STRATEGIES,
    },
  ],
} as const;
