"use client";

/**
 * useAutosave — debounced auto-save of editor state.
 *
 * Subscribes to isDirty changes via the editor API.
 * When dirty, serializes the definition and schedules a debounced save.
 * On unmount, flushes any pending save.
 *
 * Callbacks are injected by the app layer so the editor package
 * has no localStorage or Convex dependency.
 */

import { useEffect, useRef } from "react";
import { useEditor } from "../context";
import { serializeDraft } from "../draft/serializeDraft";
import { createDebouncedSave } from "../draft/createDebouncedSave";
import type { Draft } from "../draft/draftTypes";

const AUTOSAVE_DELAY_MS = 2000;

interface UseAutosaveOptions {
  /** Called with the serialized draft when auto-save fires. */
  onSave: (draft: Draft) => void;
  /** Called when loadDefinition or createBlank resets the editor. */
  onClear?: () => void;
}

function useAutosave({ onSave, onClear }: UseAutosaveOptions) {
  const editor = useEditor();
  const onSaveRef = useRef(onSave);
  const onClearRef = useRef(onClear);
  onSaveRef.current = onSave;
  onClearRef.current = onClear;

  useEffect(() => {
    const debounced = createDebouncedSave(AUTOSAVE_DELAY_MS);
    let prevIsDirty = editor.getState().isDirty;

    const unsubscribe = editor.subscribe((state) => {
      // Clear draft on loadDefinition / createBlank (isDirty goes false + nodes change)
      if (prevIsDirty && !state.isDirty) {
        debounced.cancel();
        onClearRef.current?.();
      }

      // Schedule save when editor becomes dirty or stays dirty with new changes
      if (state.isDirty) {
        debounced.schedule(() => {
          const current = editor.getState();
          const definition = editor.definition.exportAsDefinition();
          const draft = serializeDraft(definition, current.recipeMetadata);

          onSaveRef.current(draft);
          editor.definition.setLastSavedAt(draft.savedAt);
          editor.definition.resetDirty();
        });
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
