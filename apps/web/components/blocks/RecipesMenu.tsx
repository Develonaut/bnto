/**
 * Explore dropdown menu — curated recipe links from the registry.
 *
 * BookOpenIcon trigger opens a 2-column grid of featured recipes grouped by
 * category, with a "Browse all" footer linking to /explore.
 */

import { BookOpenIcon, Menu, MenuContent, MenuTrigger } from "@bnto/ui";

import { RecipesMenuFooter } from "./RecipesMenuFooter";
import { RecipesMenuGrid } from "./RecipesMenuGrid";

export function RecipesMenu() {
  return (
    <Menu>
      <MenuTrigger variant="ghost" elevation="sm" data-testid="explore-button">
        <BookOpenIcon />
        Explore
      </MenuTrigger>
      <MenuContent className="w-[28rem] p-0" offset="lg" data-testid="explore-dropdown">
        <div className="flex max-h-[calc(100vh-6rem)] flex-col overflow-hidden rounded-[inherit]">
          <div className="flex-1 overflow-y-auto p-3">
            <RecipesMenuGrid />
          </div>
          <RecipesMenuFooter />
        </div>
      </MenuContent>
    </Menu>
  );
}
