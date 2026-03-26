/**
 * Explorer dropdown menu — categorized recipe links from the engine menu.
 *
 * Used in both DesktopNav and MobileNavMenu.
 */

import { BookOpenIcon, Menu, MenuTrigger, MenuContent } from "@bnto/ui";

import { RecipesMenuGrid } from "./RecipesMenuGrid";

export function RecipesMenu() {
  return (
    <Menu>
      <MenuTrigger variant="outline" elevation="sm" data-testid="explore-button">
        <BookOpenIcon />
        Explore
      </MenuTrigger>
      <MenuContent className="w-[28rem] p-3" offset="lg" data-testid="explore-dropdown">
        <RecipesMenuGrid />
      </MenuContent>
    </Menu>
  );
}
