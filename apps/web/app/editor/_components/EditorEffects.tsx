"use client";

import { useCallback, useEffect, useRef } from "react";
import { useUnsavedWarning, useAutosave, useEditor } from "@bnto/editor";
import type { Draft } from "@bnto/editor";
import { core } from "@bnto/core";
import type { StoredRecipe } from "@bnto/core";
import { useStaleDraftSync } from "./useStaleDraftSync";

/** Convert an editor Draft to a StoredRecipe for the recipes store. */
function draftToStoredRecipe(draft: Draft): StoredRecipe {
  return {
    definition: draft.definition,
    metadata: {
      id: draft.metadata.id,
      name: draft.metadata.name,
      cloudId: draft.metadata.cloudId ?? undefined,
      type: draft.metadata.type,
      version: draft.metadata.version,
    },
    savedAt: draft.savedAt,
    syncedAt: draft.syncedAt,
  };
}

/**
 * EditorEffects — side-effect hooks that run inside EditorRoot context.
 *
 * Must be a child of EditorRoot (needs editor store access).
 * Wires: beforeunload warning + two-layer auto-save:
 *   Layer 1: recipesStore (always, immediate, auto-persists to localStorage)
 *   Layer 2: Convex sync (authed + cloudId, fire-and-forget)
 */
export function EditorEffects() {
  useUnsavedWarning();
  useStaleDraftSync();

  const editor = useEditor();
  const { isAuthenticated } = core.auth.useAuth();
  const isAuthRef = useRef(isAuthenticated);
  useEffect(() => {
    isAuthRef.current = isAuthenticated;
  }, [isAuthenticated]);

  const handleSave = useCallback(
    (draft: Draft) => {
      // Layer 1: recipes store (auto-persists to localStorage)
      core.recipes.upsert(draftToStoredRecipe(draft));

      // Layer 2: Convex sync (async, authed + saved recipe only)
      const cloudId = draft.metadata.cloudId;
      if (isAuthRef.current && cloudId) {
        editor.definition.setIsSyncing(true);
        core.recipes
          .save({ id: cloudId, name: draft.metadata.name, definition: draft.definition })
          .then(() => {
            const now = Date.now();
            editor.definition.setSyncedAt(now);
            // Update store with new syncedAt
            core.recipes.upsert(draftToStoredRecipe({ ...draft, syncedAt: now }));
            core.recipes.invalidateList();
          })
          .catch(() => {
            // Silent — store has the data. Retries on next save.
          })
          .finally(() => {
            editor.definition.setIsSyncing(false);
          });
      }
    },
    [editor],
  );

  useAutosave({ onSave: handleSave });

  return null;
}
