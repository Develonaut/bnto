/** Loop node schema — parameters for iteration (forEach, times, while). */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Valid loop execution modes. */
export const LOOP_MODES = ["forEach", "times", "while"] as const;

/** Zod schema for loop node parameters. */
export const loopParamsSchema = z.object({
  mode: z.enum(LOOP_MODES),
  items: z.string().optional(),
  count: z.number().min(0).optional(),
  condition: z.string().optional(),
  breakCondition: z.string().optional(),
});

/** Inferred TypeScript type for loop node parameters. */
export type LoopParams = z.infer<typeof loopParamsSchema>;

/** Full schema definition for the loop node type. */
export const loopNodeSchema: NodeSchemaDefinition = {
  nodeType: "loop",
  schemaVersion: 1,
  schema: loopParamsSchema,
  params: {
    mode: {
      label: "Mode",
      description: "How the loop iterates: over items, N times, or while a condition holds.",
    },
    items: {
      label: "Items",
      description:
        "Optional template expression for the item source. The Rust engine " +
        "iterates over the incoming file batch by default — this is only " +
        "needed for custom data sources.",
      placeholder: '{{index . "list-files" "files"}}',
      visibleWhen: { param: "mode", equals: "forEach" },
    },
    count: {
      label: "Count",
      description: "Number of times to repeat.",
      visibleWhen: { param: "mode", equals: "times" },
      requiredWhen: { param: "mode", equals: "times" },
    },
    condition: {
      label: "Condition",
      description: "Expr expression that must evaluate to true to continue looping.",
      placeholder: "counter < 10",
      visibleWhen: { param: "mode", equals: "while" },
      requiredWhen: { param: "mode", equals: "while" },
    },
    breakCondition: {
      label: "Break Condition",
      description: "Optional expr expression — breaks out of the loop early when true.",
      placeholder: "item.status == 'done'",
    },
  },
};
