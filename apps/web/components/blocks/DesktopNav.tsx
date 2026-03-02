/**
 * Desktop navigation links — page buttons + Explore dropdown.
 *
 * Order: My Recipes | Explore | Pricing | FAQ
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 * Extracted from Navbar for single-responsibility.
 */

import { NavButton } from "./NavButton";
import { RecipesMenu } from "./RecipesMenu";
import { PAGE_LINKS } from "./navData";

export function DesktopNav() {
  const [first, ...rest] = PAGE_LINKS;

  return (
    <div className="hidden items-center gap-2 lg:flex">
      <NavButton href={first.href}>{first.label}</NavButton>
      <RecipesMenu />
      {rest.map((link) => (
        <NavButton key={link.href} href={link.href}>
          {link.label}
        </NavButton>
      ))}
    </div>
  );
}
