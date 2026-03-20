/**
 * Infer field type metadata from a Zod schema shape entry.
 *
 * Inspects Zod's internal `_def` structure to determine the effective type,
 * enum values, and numeric constraints for a given parameter. Used by the
 * config panel to render the correct form control.
 *
 * ## Zod Type → UI Control Mapping
 *
 * | Zod Type                  | Constraint / Hint | UI Control | Component        |
 * |---------------------------|-------------------|------------|------------------|
 * | `z.enum()`                | —                 | select     | Select           |
 * | `z.boolean()`             | —                 | switch     | Switch           |
 * | `z.number()`              | min AND max       | slider     | Slider           |
 * | `z.number()`              | unbounded / one   | number     | Input[num]       |
 * | `z.string()`              | fieldConfig.control=textarea | textarea | Textarea     |
 * | `z.string()`              | —                 | text       | Input[text]      |
 * | `z.array(z.string())`     | —                 | tagPicker  | Combobox         |
 * | `z.record(z.string())`    | —                 | keyValue   | KeyValueEditor   |
 * | `z.record(z.unknown())`   | —                 | keyValue   | KeyValueEditor   |
 *
 * This mapping is the single source of truth for which UI control renders
 * for each Zod type. The `SchemaForm` component in `@bnto/editor` consumes
 * the `control` field directly — no switch-on-type needed.
 */

import type { z } from "zod";
import type { FieldConfig } from "./types";

// ---------------------------------------------------------------------------
// Zod internal helpers — typed accessors for _def properties.
//
// Zod's _def is untyped in the public API. These helpers centralize the
// trust-boundary casts so the rest of the file stays cast-free. If a Zod
// major version changes _def internals, only these functions need updating.
// ---------------------------------------------------------------------------

interface ZodDef {
  typeName?: string;
  innerType?: z.ZodTypeAny;
  type?: z.ZodTypeAny;
  valueType?: z.ZodTypeAny;
  values?: readonly string[];
  checks?: ReadonlyArray<{ kind: string; value: number }>;
}

function zodDef(zodType: z.ZodTypeAny): ZodDef {
  return zodType._def as ZodDef;
}

/**
 * UI control type — maps directly to a `@bnto/ui` component.
 *
 * - select:    `<Select>` dropdown (for enums)
 * - switch:    `<Switch>` toggle (for booleans)
 * - slider:    `<Slider>` range (for bounded numbers with both min AND max)
 * - number:    `<Input type="number">` (for unbounded numbers)
 * - text:      `<Input type="text">` (for strings, fallback)
 * - textarea:  `<Textarea>` multiline (for strings with fieldConfig.control = "textarea")
 * - tagPicker: `<Combobox>` multi-select (for z.array(z.string()))
 * - keyValue:  `<KeyValueEditor>` key→value pairs (for z.record())
 */
type FieldControl =
  | "select"
  | "switch"
  | "slider"
  | "number"
  | "text"
  | "textarea"
  | "tagPicker"
  | "keyValue";

interface FieldTypeInfo {
  /** Effective type for rendering the correct form control. */
  type: "string" | "number" | "boolean" | "enum" | "array" | "record";
  /** UI control to render — determined by type + constraints. */
  control: FieldControl;
  /** Whether the field is required (not wrapped in ZodOptional or ZodDefault). */
  required: boolean;
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
  const def = zodDef(zodType);
  if (
    def.typeName === "ZodDefault" ||
    def.typeName === "ZodOptional" ||
    def.typeName === "ZodNullable"
  ) {
    return unwrap(def.innerType!);
  }
  return zodType;
}

/**
 * Extract min/max constraints from a ZodNumber's checks array.
 */
function extractNumberChecks(zodType: z.ZodTypeAny): { min?: number; max?: number } {
  const { checks } = zodDef(zodType);
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
 * Check if a ZodArray contains string elements (z.array(z.string())).
 */
function isStringArray(zodType: z.ZodTypeAny): boolean {
  const { type: elementType } = zodDef(zodType);
  if (!elementType) return false;
  const innerElement = unwrap(elementType);
  return zodDef(innerElement).typeName === "ZodString";
}

/**
 * Check if a ZodRecord has string values (z.record(z.string())).
 * Also matches z.record(z.unknown()) for generic key-value editing.
 */
function isKeyValueRecord(zodType: z.ZodTypeAny): boolean {
  const { valueType } = zodDef(zodType);
  if (!valueType) return false;
  const innerValue = unwrap(valueType);
  const typeName = zodDef(innerValue).typeName;
  return typeName === "ZodString" || typeName === "ZodUnknown";
}

/**
 * Infer the field type info from a Zod schema shape entry.
 *
 * Returns the effective type, UI control, enum values, and numeric constraints
 * that the config panel needs to render the correct form component.
 *
 * An optional `fieldConfig` parameter allows overriding the inferred control
 * type via `fieldConfig.control` (e.g., setting `control: "textarea"` on a
 * string field).
 */
function inferFieldType(zodField: z.ZodTypeAny, fieldConfig?: FieldConfig): FieldTypeInfo {
  const outerTypeName = zodDef(zodField).typeName ?? "";
  const required = outerTypeName !== "ZodOptional" && outerTypeName !== "ZodDefault";
  const inner = unwrap(zodField);
  const innerDef = zodDef(inner);
  const typeName = innerDef.typeName;

  if (typeName === "ZodEnum") {
    return {
      type: "enum",
      control: "select",
      required,
      enumValues: innerDef.values,
    };
  }

  if (typeName === "ZodNumber") {
    const { min, max } = extractNumberChecks(inner);
    const isBounded = min !== undefined && max !== undefined;
    return {
      type: "number",
      control: isBounded ? "slider" : "number",
      required,
      min,
      max,
    };
  }

  if (typeName === "ZodBoolean") {
    return { type: "boolean", control: "switch", required };
  }

  if (typeName === "ZodArray" && isStringArray(inner)) {
    return { type: "array", control: "tagPicker", required };
  }

  if (typeName === "ZodRecord" && isKeyValueRecord(inner)) {
    return { type: "record", control: "keyValue", required };
  }

  // String fields can be overridden to textarea via fieldConfig.control
  const control = fieldConfig?.control === "textarea" ? "textarea" : "text";
  return { type: "string", control, required };
}

export { inferFieldType };
export type { FieldTypeInfo, FieldControl };
