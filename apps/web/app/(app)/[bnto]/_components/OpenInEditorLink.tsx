"use client";

import { Button, PenLineIcon } from "@bnto/ui";
import { useFeatureFlag } from "@/lib/useFeatureFlag";

/**
 * "Open in Editor" bridge button — navigates to /editor?from={slug}.
 *
 * Only visible when the `editor` feature flag is enabled.
 * Console: `__bnto__.flags.set("editor", true)` to show.
 */
export function OpenInEditorLink({ slug }: { slug: string }) {
  const editorEnabled = useFeatureFlag("editor");
  if (!editorEnabled) return null;
  return (
    <Button href={`/editor?from=${slug}`} variant="outline" elevation="sm">
      <PenLineIcon className="size-3.5" />
      Customize in Editor
    </Button>
  );
}
