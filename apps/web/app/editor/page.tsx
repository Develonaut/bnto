"use client";

import { useMemo } from "react";
import { useSearchParams, useRouter } from "next/navigation";
import { useEffect } from "react";
import { getRecipeBySlug } from "@bnto/nodes";

import {
  EditorRoot,
  EditorCanvas,
  EditorToolbar,
  EditorRightToolbar,
} from "@bnto/editor";

import { useFeatureFlag } from "@/lib/useFeatureFlag";

/**
 * /editor — full-viewport recipe editor.
 *
 * Gated behind the `editor` feature flag. When disabled, redirects to /.
 * Enable via console: `__bnto__.flags.set("editor", true)`
 *
 * Slug lookup happens here (app layer). The editor accepts a Definition
 * directly — it has no knowledge of slugs or the recipe registry.
 */
export default function EditorPage() {
  const editorEnabled = useFeatureFlag("editor");
  const searchParams = useSearchParams();
  const router = useRouter();
  const from = searchParams.get("from") ?? undefined;

  const definition = useMemo(() => {
    if (!from) return undefined;
    return getRecipeBySlug(from)?.definition;
  }, [from]);

  useEffect(() => {
    if (!editorEnabled) {
      // Defer redirect so useSyncExternalStore can settle after hydration.
      // During hydration the server snapshot (false) is used briefly before
      // the client snapshot resolves. Without the delay, the redirect fires
      // before the flag re-evaluates.
      const timer = setTimeout(() => router.replace("/"), 500);
      return () => clearTimeout(timer);
    }
  }, [editorEnabled, router]);

  if (!editorEnabled) return null;

  return (
    <EditorRoot definition={definition}>
      <EditorCanvas>
        <EditorToolbar />
        <EditorRightToolbar />
      </EditorCanvas>
    </EditorRoot>
  );
}
