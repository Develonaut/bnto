"use client";

/**
 * useStaleDraftSync — on mount, sync stale localStorage drafts to Convex.
 *
 * If the editor has a cloudId and the local draft is newer than the
 * last sync, fires a background Convex save. Runs once per mount.
 */

import { useEffect, useRef } from "react";
import { useEditor } from "@bnto/editor";
import { core } from "@bnto/core";

function useStaleDraftSync() {
  const editor = useEditor();
  const { isAuthenticated } = core.auth.useAuth();
  const hasSynced = useRef(false);

  useEffect(() => {
    if (hasSynced.current || !isAuthenticated) return;

    const state = editor.getState();
    const { recipeMetadata, syncedAt, lastSavedAt } = state;
    if (!recipeMetadata.cloudId) return;
    if (lastSavedAt === null) return;
    if (syncedAt !== null && lastSavedAt <= syncedAt) return;

    hasSynced.current = true;
    const definition = editor.definition.exportAsDefinition();
    core.recipes
      .save({ id: recipeMetadata.cloudId, name: recipeMetadata.name, definition })
      .then(() => {
        editor.definition.setSyncedAt(Date.now());
        core.recipes.invalidateList();
      })
      .catch(() => {
        // Silent — will retry on next autosave
      });
  }, [editor, isAuthenticated]);
}

export { useStaleDraftSync };
