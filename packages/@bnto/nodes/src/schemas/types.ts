/**
 * Schema types — describe the parameters each node type accepts.
 *
 * Two distinct type families:
 *
 * - **NodeParam** — Engine metadata generated from the Rust catalog.
 *   Labels, descriptions, conditional visibility/requirement rules.
 *   Lives in `NodeSchema.params`.
 *
 * - **NodeParamField / NodeParamFields** — UI presentation metadata.
 *   Hidden flags, layout grouping, presets, option labels, control overrides.
 *   Lives in `NODE_PARAM_FIELDS` alongside the schema registry.
 *
 * Zod schemas remain the single source of truth for validation.
 */

import type { z } from "zod";

/**
 * UI control type — maps directly to a `@bnto/ui` component.
 *
 * - select:           `<Select>` dropdown (for enums)
 * - switch:           `<Switch>` toggle (for booleans)
 * - slider:           `<Slider>` range (for bounded numbers with both min AND max)
 * - number:           `<Input type="number">` (for unbounded numbers)
 * - text:             `<Input type="text">` (for strings, fallback)
 * - textarea:         `<Textarea>` multiline (for strings with control = "textarea")
 * - tagPicker:        `<Combobox>` multi-select (for z.array(z.string()))
 * - keyValue:         `<KeyValueEditor>` key→value pairs (for z.record())
 * - file:             `<FileControl>` file picker → base64 data URI (for file params)
 * - positionGrid:     `<PositionGridControl>` 3×3 clickable grid (for 9-position enums)
 * - watermarkPreview: `<WatermarkPreviewControl>` live composite preview
 */
export type NodeParamControl =
  | "select"
  | "switch"
  | "slider"
  | "number"
  | "text"
  | "textarea"
  | "tagPicker"
  | "keyValue"
  | "file"
  | "positionGrid"
  | "watermarkPreview";

/** Pre-computed field type info for a node parameter — replaces runtime Zod introspection. */
export interface NodeParamFieldInfo {
  /** Effective type for rendering the correct form control. */
  type: "string" | "number" | "boolean" | "enum" | "array" | "record";
  /** UI control to render — determined by type + constraints. */
  control: NodeParamControl;
  /** Whether the field is required (not wrapped in ZodOptional or ZodDefault). */
  required: boolean;
  /** Enum values if the field is an enum. */
  enumValues?: readonly string[];
  /** Minimum value for number fields. */
  min?: number;
  /** Maximum value for number fields. */
  max?: number;
}

/** Condition for visibleWhen/requiredWhen rules — single or OR array. */
export type ParamCondition =
  | { readonly param: string; readonly equals: string }
  | ReadonlyArray<{ readonly param: string; readonly equals: string }>;

/**
 * Engine metadata for a single parameter.
 *
 * Describes what the engine knows about a parameter: its label, description,
 * and surfaceability. Generated from the Rust engine catalog. Does NOT include
 * UI presentation concerns — those live in `NodeParamField`.
 *
 * Conditional visibility and requirement rules live in `NodeParamField` so
 * that all UI-facing conditional logic is in one place.
 */
export interface NodeParam {
  /** Human-readable label for the config panel. */
  label: string;

  /** One-sentence description of what the parameter does. */
  description: string;

  /** Placeholder text for string/number inputs. */
  placeholder?: string;

  /**
   * Whether this param is eligible for surfacing in container config panels.
   * Defaults to true — only set to false for params that make no sense
   * when promoted to a parent container.
   */
  surfaceable?: boolean;
}

// ---------------------------------------------------------------------------
// NodeParamField — UI presentation metadata (separate from engine concerns)
// ---------------------------------------------------------------------------

/**
 * UI presentation config for a single parameter field.
 *
 * This is what the editor config panel needs to render a field correctly:
 * whether to hide it, how to group it, what presets to show, etc.
 *
 * Defined per-node-type in augmentation files (e.g., `image.ts`), keyed by
 * parameter name. Consumed by SchemaForm, SchemaField, and control components.
 */
export interface NodeParamField {
  /**
   * Layout group name — consecutive params with the same group
   * are rendered together in a compact FieldGroup (e.g., "dimensions"
   * renders Width + Height side-by-side with aspect lock toggle above).
   */
  group?: string;

  /** Unit suffix displayed inside the input (e.g., "px", "%", "ms"). */
  suffix?: string;

  /** Override the label shown in the config panel. */
  label?: string;

  /**
   * Display the slider with inverted semantics — the UI shows
   * (max + min - value) while the stored value stays as-is.
   */
  inverted?: boolean;

  /**
   * Slider presets — clickable named positions along the slider track.
   * Each preset maps a value to a label (e.g., { value: 80, label: "High" }).
   */
  presets?: Array<{ value: number; label: string }>;

  /**
   * Display labels for enum options. Each entry maps a stored value to a
   * human-readable label (e.g., { value: "compress", label: "Compress" }).
   */
  options?: Array<{ value: string; label: string }>;

  /**
   * Override the inferred UI control type. Used when the Zod type alone
   * isn't enough to determine the right control (e.g., a z.string() that
   * should render as a textarea instead of a single-line input).
   */
  control?: NodeParamControl;

  /**
   * Accepted MIME types for file controls. Only relevant when
   * `control` is `"file"`. Passed through to the file input's accept attribute.
   */
  accept?: string[];

  /**
   * Conditional visibility — parameter is shown only when
   * another parameter matches a specific value.
   *
   * Single condition or array of conditions (OR logic).
   */
  visibleWhen?: ParamCondition;

  /**
   * Conditional requirement — parameter is required only when
   * another parameter matches a specific value.
   *
   * Single condition or array of conditions (OR logic).
   */
  requiredWhen?: ParamCondition;
}

/** Field configs for all parameters of a node type. */
export type NodeParamFields = Record<string, NodeParamField>;

/**
 * Complete schema definition for a node type.
 *
 * Combines Zod validation schema with UI metadata and versioning.
 */
export interface NodeSchema {
  /** The node type name (e.g., "http-request", "image-compress"). */
  nodeType: string;

  /**
   * Parameter schema version — tracks changes to this node type's parameters.
   * Bump when parameters are added, removed, renamed, or have their
   * type/constraints changed.
   */
  schemaVersion: number;

  /** Zod schema for runtime validation of node parameters. */
  schema: z.ZodObject<z.ZodRawShape>;

  /** Engine metadata keyed by parameter name. */
  params: Record<string, NodeParam>;
}
