/**
 * Desktop navigation links — recipe dropdown + page buttons.
 *
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 * Extracted from Navbar for single-responsibility.
 */

import { NavButton } from "./NavButton";
import { RecipesMenu } from "./RecipesMenu";
import { PAGE_LINKS } from "./navData";

export function DesktopNav() {
  return (
    <div className="hidden items-center gap-2 lg:flex">
      <RecipesMenu />
      {PAGE_LINKS.map((link) => (
        <NavButton key={link.href} href={link.href}>
          {link.label}
        </NavButton>
      ))}
    </div>
  );
}
