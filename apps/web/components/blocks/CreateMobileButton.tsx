"use client";

/**
 * Mobile "Create" button — only visible when the `editor` feature flag is on.
 *
 * Console: `__bnto__.flags.set("editor", true)` to enable.
 */

import { Button } from "@bnto/ui";
import { useFeatureFlag } from "@/lib/useFeatureFlag";

export function CreateMobileButton({ onClick }: { onClick: () => void }) {
  const editorEnabled = useFeatureFlag("editor");
  if (!editorEnabled) return null;
  return (
    <Button variant="outline" href="/editor" onClick={onClick}>
      Create
    </Button>
  );
}
