"use client";

/**
 * useAutosave — debounced auto-save of editor state.
 *
 * Always on. Subscribes to isDirty changes via the editor API.
 * When dirty, serializes the definition and schedules a debounced save.
 * On unmount, flushes any pending save.
 *
 * Drafts are never cleared by autosave — they persist in localStorage
 * and show up in My Recipes. Explicit deletion happens from My Recipes.
 *
 * The onSave callback is injected by the app layer so the editor package
 * has no localStorage or Convex dependency.
 */

import { useEffect, useRef } from "react";
import { useEditor } from "../context";
import { serializeDraft } from "../draft/serializeDraft";
import { createDebouncedSave } from "../draft/createDebouncedSave";
import type { Draft } from "../draft/draftTypes";
import type { ReactEditorInstance } from "../reactEditorTypes";

const AUTOSAVE_DELAY_MS = 2000;

interface UseAutosaveOptions {
  /** Called with the serialized draft when auto-save fires. */
  onSave: (draft: Draft) => void;
}

function buildSaveCallback(
  editor: ReactEditorInstance,
  onSaveRef: { current: (draft: Draft) => void },
) {
  return () => {
    const current = editor.getState();
    const definition = editor.definition.exportAsDefinition();
    const draft = serializeDraft(definition, current.recipeMetadata, current.syncedAt);

    onSaveRef.current(draft);
    editor.definition.setLastSavedAt(draft.savedAt);
    editor.definition.resetDirty();
  };
}

function useAutosave({ onSave }: UseAutosaveOptions) {
  const editor = useEditor();
  const onSaveRef = useRef(onSave);
  onSaveRef.current = onSave;

  useEffect(() => {
    const debounced = createDebouncedSave(AUTOSAVE_DELAY_MS);
    const performSave = buildSaveCallback(editor, onSaveRef);
    let prevIsDirty = editor.getState().isDirty;

    const unsubscribe = editor.subscribe((state) => {
      // Cancel pending save when dirty→clean (loadDefinition / createBlank)
      if (prevIsDirty && !state.isDirty) {
        debounced.cancel();
      }

      // Schedule save when dirty
      if (state.isDirty) {
        debounced.schedule(performSave);
      }

      prevIsDirty = state.isDirty;
    });

    return () => {
      debounced.flush();
      debounced.destroy();
      unsubscribe();
    };
  }, [editor]);
}

export { useAutosave };
export type { UseAutosaveOptions };
