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
