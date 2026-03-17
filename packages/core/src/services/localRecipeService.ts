"use client";

import { draftToRecipeListItem } from "../transforms/localRecipe";
import type { LocalDraft } from "../transforms/localRecipe";
import type { RecipeListItem } from "../types";

/**
 * Key prefix matching @bnto/editor's draftStorage format.
 * Kept in sync manually — the editor writes, core reads.
 */
const DRAFT_KEY_PREFIX = "bnto_editor_draft";

/** Parse a raw localStorage value into a LocalDraft, or null if corrupted. */
function parseDraft(raw: string): LocalDraft | null {
  try {
    const parsed = JSON.parse(raw);
    if (!parsed?.metadata?.id || typeof parsed.savedAt !== "number") return null;
    return parsed as LocalDraft;
  } catch {
    return null;
  }
}

/** Read all drafts from localStorage, sorted newest-first. */
function listDrafts(storage: Storage): LocalDraft[] {
  const drafts: LocalDraft[] = [];
  for (let i = 0; i < storage.length; i++) {
    const key = storage.key(i);
    if (!key || !key.startsWith(`${DRAFT_KEY_PREFIX}:`)) continue;
    if (key === `${DRAFT_KEY_PREFIX}:_latest`) continue;

    const raw = storage.getItem(key);
    if (!raw) continue;
    const draft = parseDraft(raw);
    if (draft) drafts.push(draft);
  }
  return drafts.sort((a, b) => b.savedAt - a.savedAt);
}

/**
 * Local recipe service — localStorage-backed recipe operations.
 *
 * Reads drafts written by @bnto/editor's draftStorage and exposes
 * them as RecipeListItem[] — the same shape used by cloud recipes.
 * The app layer never touches localStorage directly.
 */
export function createLocalRecipeService() {
  function getStorage(): Storage | null {
    return typeof window !== "undefined" ? localStorage : null;
  }

  return {
    list: (): RecipeListItem[] => {
      const storage = getStorage();
      if (!storage) return [];
      return listDrafts(storage).map(draftToRecipeListItem);
    },

    remove: (recipeId: string): void => {
      const storage = getStorage();
      if (!storage) return;
      storage.removeItem(`${DRAFT_KEY_PREFIX}:${recipeId}`);
    },

    count: (): number => {
      const storage = getStorage();
      if (!storage) return 0;
      return listDrafts(storage).length;
    },
  } as const;
}

export type LocalRecipeService = ReturnType<typeof createLocalRecipeService>;
