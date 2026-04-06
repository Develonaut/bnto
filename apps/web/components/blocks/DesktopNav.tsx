/**
 * Desktop navigation links — Explore dropdown + Editor button.
 *
 * Order: [Explore dropdown] | [Editor (beta)]
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 */

import { RecipesMenu } from "./RecipesMenu";
import { NewRecipeNavButton } from "./NewRecipeNavButton";

export function DesktopNav() {
  return (
    <div className="hidden items-center gap-2 lg:flex">
      <RecipesMenu />
      <NewRecipeNavButton />
    </div>
  );
}
