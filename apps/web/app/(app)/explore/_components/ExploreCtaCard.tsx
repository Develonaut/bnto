/**
 * "Create Your Own" CTA card at the end of the explore grid.
 */

import { PlusIcon, RecipeCard, RecipeCardContent, RecipeCardTitle } from "@bnto/ui";

export function ExploreCtaCard() {
  return (
    <RecipeCard href="/editor" color="primary">
      <div className="flex flex-1 flex-col items-center justify-center gap-3 py-8">
        <div className="rounded-full bg-white/20 p-3">
          <PlusIcon className="size-6" />
        </div>
        <RecipeCardContent>
          <RecipeCardTitle>Create Your Own</RecipeCardTitle>
        </RecipeCardContent>
      </div>
    </RecipeCard>
  );
}
