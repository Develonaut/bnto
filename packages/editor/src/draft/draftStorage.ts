/**
 * draftStorage — localStorage CRUD for editor drafts.
 *
 * Accepts a Storage interface so tests can inject a mock.
 * Returns null for corrupted or missing data.
 */

import type { Draft } from "./draftTypes";
import { deserializeDraft } from "./deserializeDraft";

const DRAFT_KEY = "bnto_editor_draft";

function saveDraft(storage: Storage, draft: Draft): void {
  storage.setItem(DRAFT_KEY, JSON.stringify(draft));
}

function loadDraft(storage: Storage): Draft | null {
  const raw = storage.getItem(DRAFT_KEY);
  if (!raw) return null;
  return deserializeDraft(raw);
}

function clearDraft(storage: Storage): void {
  storage.removeItem(DRAFT_KEY);
}

export { DRAFT_KEY, saveDraft, loadDraft, clearDraft };
