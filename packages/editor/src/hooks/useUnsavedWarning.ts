"use client";

import { useEffect } from "react";
import { useEditor } from "../context";

/**
 * useUnsavedWarning — shows browser beforeunload prompt when editor has
 * unsaved changes.
 *
 * Attaches a `beforeunload` event listener that fires the native browser
 * confirmation dialog when the user tries to close/refresh the tab while
 * `isDirty` is true. The browser controls the dialog text — we can't
 * customize it.
 *
 * For in-app Next.js navigation, the App Router doesn't expose route
 * change events. The beforeunload handler covers tab close, refresh,
 * and external navigation. In-app soft navigation (Link clicks, router.push)
 * doesn't trigger beforeunload — that's acceptable for v1 since the editor
 * is full-viewport and users typically don't navigate away mid-edit.
 */
export function useUnsavedWarning() {
  const editor = useEditor();
  const { isDirty } = editor.definition.useDefinition();

  useEffect(() => {
    if (!isDirty) return;

    const handler = (e: BeforeUnloadEvent) => {
      e.preventDefault();
    };

    window.addEventListener("beforeunload", handler);
    return () => window.removeEventListener("beforeunload", handler);
  }, [isDirty]);
}
