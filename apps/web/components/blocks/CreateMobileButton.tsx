"use client";

/**
 * Mobile "Create" button with Beta badge.
 *
 * Links to the recipe editor at /editor.
 */

import { Badge, Button } from "@bnto/ui";

export function CreateMobileButton({ onClick }: { onClick: () => void }) {
  return (
    <Button variant="outline" href="/editor" onClick={onClick}>
      Create <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
