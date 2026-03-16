/**
 * deserializeDraft — parse a JSON string into a Draft or null.
 *
 * Returns null for empty strings, invalid JSON, or objects
 * missing required fields (definition, metadata, savedAt).
 */

import type { Draft } from "./draftTypes";

function deserializeDraft(json: string): Draft | null {
  if (!json) return null;

  try {
    const parsed: unknown = JSON.parse(json);
    if (!parsed || typeof parsed !== "object") return null;

    const obj = parsed as Record<string, unknown>;
    if (!obj.definition || typeof obj.definition !== "object") return null;
    if (!obj.metadata || typeof obj.metadata !== "object") return null;
    if (typeof obj.savedAt !== "number") return null;

    // Backward compat: old drafts lack syncedAt
    const syncedAt = typeof obj.syncedAt === "number" ? obj.syncedAt : null;
    return { ...(parsed as Draft), syncedAt };
  } catch {
    return null;
  }
}

export { deserializeDraft };
