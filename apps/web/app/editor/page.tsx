"use client";

import { useSearchParams, useRouter } from "next/navigation";
import { useEffect } from "react";

import {
  EditorRoot,
  EditorCanvas,
  EditorToolbar,
  EditorLeftToolbar,
  EditorRightToolbar,
} from "@bnto/editor";

import { useFeatureFlag } from "@/lib/useFeatureFlag";

/**
 * /editor — full-viewport recipe editor.
 *
 * Gated behind the `editor` feature flag. When disabled, redirects to /.
 * Enable via console: `__bnto__.flags.set("editor", true)`
 */
export default function EditorPage() {
  const editorEnabled = useFeatureFlag("editor");
  const searchParams = useSearchParams();
  const router = useRouter();
  const from = searchParams.get("from") ?? undefined;

  useEffect(() => {
    if (!editorEnabled) router.replace("/");
  }, [editorEnabled, router]);

  if (!editorEnabled) return null;

  return (
    <EditorRoot slug={from}>
      <EditorCanvas>
        <EditorLeftToolbar />
        <EditorToolbar />
        <EditorRightToolbar />
      </EditorCanvas>
    </EditorRoot>
  );
}
