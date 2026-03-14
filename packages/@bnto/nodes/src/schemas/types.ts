/**
 * Schema types — describe the parameters each node type accepts.
 *
 * Zod schemas are the single source of truth for validation.
 * NodeParamMeta provides UI metadata (labels, descriptions, visibility rules)
 * that pairs with each Zod field.
 */

import type { z } from "zod";

/** Condition for visibleWhen/requiredWhen rules — single or OR array. */
export type ParamCondition =
  | { param: string; equals: string }
  | Array<{ param: string; equals: string }>;

/**
 * UI metadata for a single parameter.
 *
 * This is the information the config panel UI needs that Zod can't express:
 * labels, descriptions, placeholders, conditional visibility/requirement.
 */
export interface NodeParamMeta {
  /** Human-readable label for the config panel. */
  label: string;

  /** One-sentence description of what the parameter does. */
  description: string;

  /** Placeholder text for string/number inputs. */
  placeholder?: string;

  /**
   * Conditional visibility — parameter is shown only when
   * another parameter matches a specific value.
   */
  visibleWhen?: ParamCondition;

  /**
   * Conditional requirement — parameter is required only when
   * another parameter matches a specific value.
   */
  requiredWhen?: ParamCondition;

  /**
   * Hidden from the config panel entirely. Used for engine wiring
   * fields (e.g., input/output path templates) that the editor
   * handles implicitly via node placement.
   */
  hidden?: boolean;

  /**
   * Layout group name — consecutive params with the same group
   * are rendered together in a compact FieldGroup (e.g., "dimensions"
   * renders Width + Height side-by-side with aspect lock toggle above).
   */
  group?: string;

  /** Unit suffix displayed inside the input (e.g., "px", "%", "ms"). */
  suffix?: string;

  /**
   * Slider presets — clickable named positions along the slider track.
   * Each preset maps a value to a label (e.g., { value: 80, label: "High" }).
   * When presets are present, the numeric value display is replaced by preset labels.
   */
  presets?: Array<{ value: number; label: string }>;

  /**
   * Display the slider with inverted semantics — the UI shows (max + min - value)
   * while the stored value stays as-is. Used when the engine param and user
   * mental model are inverted (e.g., compression 1-100 displayed as quality).
   */
  displayInverted?: boolean;

  /** Override the label shown in the config panel (e.g., show "Quality" for "compression"). */
  displayLabel?: string;

  /**
   * Display labels for enum options. Each entry maps a stored value to a
   * human-readable label (e.g., { value: "compress", label: "Compress" }).
   * When present, SelectControl uses these labels instead of raw enum values.
   */
  options?: Array<{ value: string; label: string }>;

  /**
   * Override the inferred UI control type. Used when the Zod type alone
   * isn't enough to determine the right control (e.g., a z.string() that
   * should render as a textarea instead of a single-line input).
   */
  control?: "textarea";
}

/**
 * Complete schema definition for a node type.
 *
 * Combines Zod validation schema with UI metadata and versioning.
 */
export interface NodeSchemaDefinition {
  /** The node type name (e.g., "http-request", "image"). */
  nodeType: string;

  /**
   * Parameter schema version — tracks changes to this node type's parameters.
   * Bump when parameters are added, removed, renamed, or have their
   * type/constraints changed.
   */
  schemaVersion: number;

  /** Zod schema for runtime validation of node parameters. */
  schema: z.ZodObject<z.ZodRawShape>;

  /** UI metadata keyed by parameter name — drives the config panel. */
  params: Record<string, NodeParamMeta>;
}
