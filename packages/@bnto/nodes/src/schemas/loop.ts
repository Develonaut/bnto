/**
 * Loop node schema — parameters for iteration (forEach, times, while).
 *
 * Go source: engine/pkg/node/library/loop/loop.go
 * Validator: engine/pkg/validator/validators.go → validateLoop
 */

import type { NodeSchema } from "./types";

/** Valid loop execution modes. */
export const LOOP_MODES = ["forEach", "times", "while"] as const;

export const loopSchema: NodeSchema = {
  nodeType: "loop",
  schemaVersion: 1,
  parameters: [
    {
      name: "mode",
      type: "enum",
      required: true,
      label: "Mode",
      description: "How the loop iterates: over items, N times, or while a condition holds.",
      enumValues: LOOP_MODES,
    },
    {
      name: "items",
      type: "string",
      required: false,
      label: "Items",
      description: "Template expression resolving to an array to iterate over.",
      placeholder: '{{index . "list-files" "files"}}',
      visibleWhen: { param: "mode", equals: "forEach" },
      requiredWhen: { param: "mode", equals: "forEach" },
    },
    {
      name: "count",
      type: "number",
      required: false,
      label: "Count",
      description: "Number of times to repeat.",
      min: 0,
      visibleWhen: { param: "mode", equals: "times" },
      requiredWhen: { param: "mode", equals: "times" },
    },
    {
      name: "condition",
      type: "string",
      required: false,
      label: "Condition",
      description: "Expr expression that must evaluate to true to continue looping.",
      placeholder: "counter < 10",
      visibleWhen: { param: "mode", equals: "while" },
      requiredWhen: { param: "mode", equals: "while" },
    },
    {
      name: "breakCondition",
      type: "string",
      required: false,
      label: "Break Condition",
      description: "Optional expr expression — breaks out of the loop early when true.",
      placeholder: "item.status == 'done'",
    },
  ],
} as const;
