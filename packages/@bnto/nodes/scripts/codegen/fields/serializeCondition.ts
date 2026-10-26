import type { ParamConditionRaw } from "../types";

/** Serialize a ParamConditionRaw to TypeScript literal. */
export function serializeCondition(cond: ParamConditionRaw): string {
  if (Array.isArray(cond)) {
    const entries = cond.map(
      (c) => `{ param: ${JSON.stringify(c.param)}, equals: ${JSON.stringify(c.equals)} }`,
    );
    return `[${entries.join(", ")}]`;
  }
  return `{ param: ${JSON.stringify(cond.param)}, equals: ${JSON.stringify(cond.equals)} }`;
}
