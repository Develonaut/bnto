/**
 * Schema types — describe the parameters each node type accepts.
 *
 * These types power the config panel UI (Atomiton `createFieldsFromSchema` pattern).
 * ~70-80% of parameter fields need zero UI code — the schema drives the form.
 */

/** Supported parameter value types for schema definitions. */
export type ParameterType =
  | "string"
  | "number"
  | "boolean"
  | "enum"
  | "object"
  | "array";

/**
 * Describes a single parameter a node type accepts.
 *
 * Captures everything the config panel UI needs to render a form field:
 * type, label, description, constraints, and defaults.
 */
export interface ParameterSchema {
  /** Parameter key as used in `.bnto.json` `parameters` object. */
  name: string;

  /** Value type — drives which form control the UI renders. */
  type: ParameterType;

  /** Whether the parameter must be provided. */
  required: boolean;

  /** Human-readable label for the config panel. */
  label: string;

  /** One-sentence description of what the parameter does. */
  description: string;

  /** Default value when the parameter is omitted. */
  default?: unknown;

  /** Valid values for `"enum"` type parameters. */
  enumValues?: readonly string[];

  /** Minimum value for `"number"` type parameters. */
  min?: number;

  /** Maximum value for `"number"` type parameters. */
  max?: number;

  /**
   * Conditional requirement — parameter is required only when
   * another parameter matches a specific value.
   *
   * Example: `items` is required only when `mode` is `"forEach"`.
   */
  requiredWhen?: {
    /** The parameter name to check. */
    param: string;
    /** The value that triggers the requirement. */
    equals: string;
  };

  /**
   * Conditional visibility — parameter is shown only when
   * another parameter matches a specific value.
   *
   * Example: `base` and `overlay` are shown only when
   * `operation` is `"composite"`.
   */
  visibleWhen?: {
    /** The parameter name to check. */
    param: string;
    /** The value that triggers visibility. */
    equals: string;
  };

  /** Placeholder text for string inputs. */
  placeholder?: string;
}

/**
 * Describes all parameters for a single node type.
 *
 * One `NodeSchema` per node type — drives the entire config panel form.
 */
export interface NodeSchema {
  /** The node type name (e.g., "http-request", "image"). */
  nodeType: string;

  /** All parameters this node type accepts. */
  parameters: readonly ParameterSchema[];
}
