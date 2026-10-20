/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for assert node parameters. */
export const assertParamsSchema = z.object({
  value: z.string(),
  op: z.enum(["equals", "not_equals", "after", "before"] as const).default("equals"),
  expected: z.string().optional(),
  message: z.string().optional(),
});

/** Inferred TypeScript type for assert node parameters. */
export type AssertParams = z.infer<typeof assertParamsSchema>;

/** Full schema definition for the assert node type. */
export const assertNodeSchema: NodeSchema = {
  nodeType: "assert",
  schemaVersion: 1,
  schema: assertParamsSchema,
  params: {
    value: {
      label: "Value",
      description:
        "What to check, usually a template such as {{node.<id>.<key>}}. With {{item.<key>}} or {{file}} (the file's name) it is checked once per incoming file, and no files means nothing to check.",
    },
    op: {
      label: "Comparison",
      description: "How value is compared to expected. after and before compare RFC 3339 times.",
    },
    expected: {
      label: "Expected",
      description: "What the value is compared to.",
    },
    message: {
      label: "Message",
      description:
        "Put before the reasons when the check fails, so the failure says what was checked.",
    },
  },
};
