/**
 * Desktop navigation links — recipe dropdown + page buttons.
 *
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 * Extracted from Navbar for single-responsibility.
 */

"use client";

import { useEffect, useState } from "react";

import { Button } from "@/components/ui/Button";

import { RecipesMenu } from "./RecipesMenu";
import { PAGE_LINKS } from "./navData";

export function DesktopNav({ pathname }: { pathname: string }) {
  const [pendingHref, setPendingHref] = useState<string | null>(null);

  // Clear pending state once navigation completes
  useEffect(() => {
    setPendingHref(null);
  }, [pathname]);

  return (
    <div className="hidden items-center gap-2 lg:flex">
      <RecipesMenu />
      {PAGE_LINKS.map((link) => (
        <Button
          key={link.href}
          variant="outline"
          elevation="sm"
          href={link.href}
          pressed={pathname === link.href || pendingHref === link.href}
          onClick={() => setPendingHref(link.href)}
        >
          {link.label}
        </Button>
      ))}
    </div>
  );
}
