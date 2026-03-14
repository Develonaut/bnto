"use client";

/**
 * Mobile "New Recipe" button with Beta badge.
 *
 * Links to the recipe editor at /editor.
 */

import { Badge, Button } from "@bnto/ui";

export function NewRecipeMobileButton({ onClick }: { onClick: () => void }) {
  return (
    <Button variant="outline" href="/editor" onClick={onClick}>
      New Recipe <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
