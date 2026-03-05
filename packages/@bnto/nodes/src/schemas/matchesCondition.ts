/**
 * Condition matcher for requiredWhen/visibleWhen fields.
 *
 * Handles both single condition objects and arrays of conditions (OR logic).
 */

import type { ParamCondition } from "./types";

/**
 * Checks if a condition (single or array) matches the given param/value pair.
 * Array conditions use OR logic -- any match returns true.
 */
export function matchesCondition(
  condition: ParamCondition | undefined,
  paramName: string,
  paramValue: string,
): boolean {
  if (!condition) return false;
  if (Array.isArray(condition)) {
    return condition.some((c) => c.param === paramName && c.equals === paramValue);
  }
  return condition.param === paramName && condition.equals === paramValue;
}
