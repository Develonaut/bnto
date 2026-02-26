/**
 * Desktop navigation links — recipe dropdown + page buttons.
 *
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 * Extracted from Navbar for single-responsibility.
 */

import { Button } from "@/components/ui/Button";

import { RecipesMenu } from "./RecipesMenu";
import { PAGE_LINKS } from "./navData";

export function DesktopNav({ pathname }: { pathname: string }) {
  return (
    <div className="hidden items-center gap-2 lg:flex">
      <RecipesMenu />
      {PAGE_LINKS.map((link) => (
        <Button
          key={link.href}
          variant="outline"
          href={link.href}
          depth="sm"
          pressed={pathname === link.href}
        >
          {link.label}
        </Button>
      ))}
    </div>
  );
}
