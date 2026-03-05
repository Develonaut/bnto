import { Button, PenLineIcon } from "@bnto/ui";

/**
 * "Open in Editor" bridge button — navigates to /editor?from={slug}.
 *
 * The editor is free for all users, so this is a simple link.
 */
export function OpenInEditorLink({ slug }: { slug: string }) {
  return (
    <Button href={`/editor?from=${slug}`} variant="outline" elevation="sm">
      <PenLineIcon className="size-3.5" />
      Customize in Editor
    </Button>
  );
}
