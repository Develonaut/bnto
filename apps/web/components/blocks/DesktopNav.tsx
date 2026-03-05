/**
 * Desktop navigation links — page buttons + Explore dropdown.
 *
 * Order: Create | Explore | Pricing | FAQ
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 */

import { NavButton } from "./NavButton";
import { RecipesMenu } from "./RecipesMenu";
import { PAGE_LINKS } from "./navData";

export function DesktopNav() {
  return (
    <div className="hidden items-center gap-2 lg:flex">
      <NavButton href="/editor">Create</NavButton>
      <RecipesMenu />
      {PAGE_LINKS.map((link) => (
        <NavButton key={link.href} href={link.href}>
          {link.label}
        </NavButton>
      ))}
    </div>
  );
}
