"use client";

/**
 * Mobile "Editor" button with Beta badge.
 *
 * Links to the recipe editor at /editor.
 */

import { Badge, Button } from "@bnto/ui";

export function NewRecipeMobileButton({ onClick }: { onClick: () => void }) {
  return (
    <Button variant="outline" href="/editor" onClick={onClick}>
      Editor <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
