"use client";

import { Surface, Grid } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { BentoRecipeFlow } from "../../_hooks/useBentoRecipeFlow";
import { RecipeGridLeft } from "./RecipeGridLeft";
import { RecipeGridRight } from "./RecipeGridRight";

interface RecipeGridProps {
  entry: BntoEntry;
  flow: BentoRecipeFlow;
}

/** The outer bento box — Surface + Grid composing compartment cards. */
export function RecipeGrid({ entry, flow }: RecipeGridProps) {
  return (
    <Surface elevation="none" border="dashed" rounded="2xl" className="p-4">
      <Grid cols={{ mobile: 1, tablet: 2, desktop: 6 }} rows={6} gap="md" animated>
        <RecipeGridLeft entry={entry} flow={flow} />
        <RecipeGridRight flow={flow} />
      </Grid>
    </Surface>
  );
}
