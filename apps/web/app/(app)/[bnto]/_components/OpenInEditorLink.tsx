"use client";

import { Badge, Button, PenLineIcon } from "@bnto/ui";

/**
 * "Open in Editor" bridge button — navigates to /editor?from={slug}.
 *
 * Shows a Beta badge to set expectations while the editor is in beta.
 */
export function OpenInEditorLink({ slug }: { slug: string }) {
  return (
    <Button href={`/editor?from=${slug}`} variant="outline" elevation="sm">
      <PenLineIcon className="size-3.5" />
      Open in Editor
      <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
