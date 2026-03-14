"use client";

import { useUnsavedWarning } from "@bnto/editor";

/**
 * EditorEffects — side-effect hooks that run inside EditorRoot context.
 *
 * Must be a child of EditorRoot (needs editor store access).
 * Currently wires: beforeunload warning for unsaved changes.
 */
export function EditorEffects() {
  useUnsavedWarning();
  return null;
}
