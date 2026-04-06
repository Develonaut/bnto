"use client";

/**
 * "Editor" nav button with Beta badge.
 *
 * Links to the recipe editor at /editor.
 */

import { Badge } from "@bnto/ui";
import { NavButton } from "./NavButton";

export function NewRecipeNavButton() {
  return (
    <NavButton href="/editor">
      Editor <Badge variant="secondary">Beta</Badge>
    </NavButton>
  );
}
