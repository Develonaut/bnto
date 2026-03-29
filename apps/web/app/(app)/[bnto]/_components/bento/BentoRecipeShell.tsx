"use client";

import type { BntoEntry } from "@/lib/bntoRegistry";
import { useBentoRecipeFlow } from "../../_hooks/useBentoRecipeFlow";
import { SessionMarker } from "../SessionMarker";
import { RecipeGrid } from "./RecipeGrid";

interface BentoRecipeShellProps {
  entry: BntoEntry;
}

/** Top-level bento grid shell — client component toggle target. */
export function BentoRecipeShell({ entry }: BentoRecipeShellProps) {
  const flow = useBentoRecipeFlow(entry.slug);

  return (
    <div data-testid="bento-shell" data-session="ready">
      <SessionMarker />
      <RecipeGrid entry={entry} flow={flow} />
    </div>
  );
}
