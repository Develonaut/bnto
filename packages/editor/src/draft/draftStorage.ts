/**
 * draftStorage — localStorage CRUD for editor drafts.
 *
 * Per-recipe keys: `bnto_editor_draft:<recipeId>`.
 * A `_latest` index tracks the most recently saved recipe ID
 * so hydration can find the right draft without scanning.
 *
 * Accepts a Storage interface so tests can inject a mock.
 * Returns null for corrupted or missing data.
 */

import type { Draft } from "./draftTypes";
import { deserializeDraft } from "./deserializeDraft";

const DRAFT_KEY_PREFIX = "bnto_editor_draft";

/** Build a per-recipe draft key. */
function draftKey(recipeId: string): string {
  return `${DRAFT_KEY_PREFIX}:${recipeId}`;
}

function saveDraft(storage: Storage, draft: Draft): void {
  const key = draftKey(draft.metadata.id);
  storage.setItem(key, JSON.stringify(draft));
  storage.setItem(`${DRAFT_KEY_PREFIX}:_latest`, draft.metadata.id);
}

function loadDraft(storage: Storage, recipeId?: string): Draft | null {
  const id = recipeId ?? storage.getItem(`${DRAFT_KEY_PREFIX}:_latest`);
  if (!id) return null;
  const raw = storage.getItem(draftKey(id));
  if (!raw) return null;
  return deserializeDraft(raw);
}

function clearDraft(storage: Storage, recipeId?: string): void {
  if (recipeId) {
    storage.removeItem(draftKey(recipeId));
  } else {
    const id = storage.getItem(`${DRAFT_KEY_PREFIX}:_latest`);
    if (id) storage.removeItem(draftKey(id));
  }
  storage.removeItem(`${DRAFT_KEY_PREFIX}:_latest`);
}

/**
 * Enumerate all saved drafts, sorted newest-first by `savedAt`.
 *
 * Scans Storage keys for the draft prefix, deserializes each,
 * and skips the `_latest` index pointer and corrupted entries.
 */
function listAllDrafts(storage: Storage): Draft[] {
  const drafts: Draft[] = [];
  for (let i = 0; i < storage.length; i++) {
    const key = storage.key(i);
    if (!key || !key.startsWith(`${DRAFT_KEY_PREFIX}:`)) continue;
    if (key === `${DRAFT_KEY_PREFIX}:_latest`) continue;

    const raw = storage.getItem(key);
    if (!raw) continue;
    const draft = deserializeDraft(raw);
    if (draft) drafts.push(draft);
  }
  return drafts.sort((a, b) => b.savedAt - a.savedAt);
}

export { DRAFT_KEY_PREFIX, draftKey, saveDraft, loadDraft, clearDraft, listAllDrafts };
