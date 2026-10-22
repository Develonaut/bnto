/**
 * "macOS only"-style badge for a CLI recipe that runs on some OSes only.
 * Renders nothing for a browser recipe or one that runs everywhere. Reads
 * `@bnto/registry` (stateless) so server pages can render it too.
 */

import type { Recipe } from "@bnto/core";
import { getRecipeOsBadge } from "@bnto/registry";
import { Badge } from "@bnto/ui";

export function RecipeOsBadge({ recipe }: { recipe: Recipe }) {
  const label = getRecipeOsBadge(recipe);
  if (!label) return null;
  return (
    <Badge variant="outline" data-testid={`recipe-os-badge-${recipe.slug}`}>
      {label}
    </Badge>
  );
}
