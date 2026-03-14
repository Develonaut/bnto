/**
 * Desktop navigation links — page buttons + Explore dropdown.
 *
 * Order: [New Recipe (flagged)] | Explore | Pricing | FAQ
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 */

import { NewRecipeNavButton } from "./NewRecipeNavButton";
import { NavButton } from "./NavButton";
import { RecipesMenu } from "./RecipesMenu";
import { PAGE_LINKS } from "./navData";

export function DesktopNav() {
  return (
    <div className="hidden items-center gap-2 lg:flex">
      <NewRecipeNavButton />
      <RecipesMenu />
      {PAGE_LINKS.map((link) => (
        <NavButton key={link.href} href={link.href}>
          {link.label}
        </NavButton>
      ))}
    </div>
  );
}
