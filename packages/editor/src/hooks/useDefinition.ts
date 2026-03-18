/**
 * useDefinition — standalone domain hook for definition state + actions.
 *
 * Delegates to editor.definition.useDefinition() for state. Actions come
 * directly from the editor instance. Prefer editor.definition.useDefinition()
 * in new code.
 */

"use client";

import { useEditor } from "../context";

function useDefinition() {
  const editor = useEditor();
  const state = editor.definition.useDefinition();

  return {
    ...state,
    loadDefinition: editor.definition.loadDefinition,
    createBlank: editor.definition.createBlank,
    updateParams: editor.definition.updateParams,
    exportAsDefinition: editor.definition.exportAsDefinition,
    exportAsRecipe: editor.definition.exportAsRecipe,
    setRecipeMetadata: editor.definition.setRecipeMetadata,
    revalidate: editor.definition.revalidate,
  };
}

export { useDefinition };
