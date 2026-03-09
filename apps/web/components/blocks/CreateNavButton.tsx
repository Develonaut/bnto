"use client";

/**
 * "Create" nav button — only visible when the `editor` feature flag is on.
 *
 * Console: `__bnto__.flags.set("editor", true)` to enable.
 */

import { useFeatureFlag } from "@/lib/useFeatureFlag";
import { NavButton } from "./NavButton";

export function CreateNavButton() {
  const editorEnabled = useFeatureFlag("editor");
  if (!editorEnabled) return null;
  return <NavButton href="/editor">Create</NavButton>;
}
