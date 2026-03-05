/**
 * Infer field type metadata from a Zod schema shape entry.
 *
 * Inspects Zod's internal `_def` structure to determine the effective type,
 * enum values, and numeric constraints for a given parameter. Used by the
 * config panel to render the correct form control.
 *
 * ## Zod Type → UI Control Mapping
 *
 * | Zod Type           | Constraint        | UI Control | Component   |
 * |--------------------|-------------------|------------|-------------|
 * | `z.enum()`         | —                 | select     | Select      |
 * | `z.boolean()`      | —                 | switch     | Switch      |
 * | `z.number()`       | min AND max       | slider     | Slider      |
 * | `z.number()`       | unbounded / one   | number     | Input[num]  |
 * | `z.string()`       | —                 | text       | Input[text] |
 *
 * This mapping is the single source of truth for which UI control renders
 * for each Zod type. The `SchemaForm` component in `@bnto/editor` consumes
 * the `control` field directly — no switch-on-type needed.
 */

import type { z } from "zod";

/**
 * UI control type — maps directly to a `@bnto/ui` component.
 *
 * - select:  `<Select>` dropdown (for enums)
 * - switch:  `<Switch>` toggle (for booleans)
 * - slider:  `<Slider>` range (for bounded numbers with both min AND max)
 * - number:  `<Input type="number">` (for unbounded numbers)
 * - text:    `<Input type="text">` (for strings, fallback)
 */
type FieldControl = "select" | "switch" | "slider" | "number" | "text";

interface FieldTypeInfo {
  /** Effective type for rendering the correct form control. */
  type: "string" | "number" | "boolean" | "enum";
  /** UI control to render — determined by type + constraints. */
  control: FieldControl;
  /** Enum values if the field is an enum. */
  enumValues?: readonly string[];
  /** Minimum value for number fields. */
  min?: number;
  /** Maximum value for number fields. */
  max?: number;
}

/**
 * Unwrap optional/default/nullable wrappers to get the inner Zod type.
 *
 * Zod wraps types in layers: ZodDefault → ZodOptional → ZodNumber.
 * We peel those off to find the core type.
 */
function unwrap(zodType: z.ZodTypeAny): z.ZodTypeAny {
  const def = zodType._def;
  if (
    def.typeName === "ZodDefault" ||
    def.typeName === "ZodOptional" ||
    def.typeName === "ZodNullable"
  ) {
    return unwrap(def.innerType);
  }
  return zodType;
}

/**
 * Extract min/max constraints from a ZodNumber's checks array.
 */
function extractNumberChecks(zodType: z.ZodTypeAny): { min?: number; max?: number } {
  const checks = zodType._def.checks as Array<{ kind: string; value: number }> | undefined;
  if (!checks) return {};
  let min: number | undefined;
  let max: number | undefined;
  for (const check of checks) {
    if (check.kind === "min") min = check.value;
    if (check.kind === "max") max = check.value;
  }
  return { min, max };
}

/**
 * Infer the field type info from a Zod schema shape entry.
 *
 * Returns the effective type, UI control, enum values, and numeric constraints
 * that the config panel needs to render the correct form component.
 */
function inferFieldType(zodField: z.ZodTypeAny): FieldTypeInfo {
  const inner = unwrap(zodField);
  const typeName = inner._def.typeName as string;

  if (typeName === "ZodEnum") {
    return {
      type: "enum",
      control: "select",
      enumValues: inner._def.values as readonly string[],
    };
  }

  if (typeName === "ZodNumber") {
    const { min, max } = extractNumberChecks(inner);
    const isBounded = min !== undefined && max !== undefined;
    return {
      type: "number",
      control: isBounded ? "slider" : "number",
      min,
      max,
    };
  }

  if (typeName === "ZodBoolean") {
    return { type: "boolean", control: "switch" };
  }

  return { type: "string", control: "text" };
}

export { inferFieldType };
export type { FieldTypeInfo, FieldControl };
