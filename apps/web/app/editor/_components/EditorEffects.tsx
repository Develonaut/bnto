"use client";

import { useCallback } from "react";
import { useUnsavedWarning, useAutosave, saveDraft, clearDraft } from "@bnto/editor";
import type { Draft } from "@bnto/editor";

/**
 * EditorEffects — side-effect hooks that run inside EditorRoot context.
 *
 * Must be a child of EditorRoot (needs editor store access).
 * Wires: beforeunload warning + debounced auto-save to localStorage.
 */
export function EditorEffects() {
  useUnsavedWarning();

  const handleSave = useCallback((draft: Draft) => {
    saveDraft(localStorage, draft);
  }, []);

  const handleClear = useCallback(() => {
    clearDraft(localStorage);
  }, []);

  useAutosave({ onSave: handleSave, onClear: handleClear });

  return null;
}
