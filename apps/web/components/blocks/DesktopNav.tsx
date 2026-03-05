"use client";

/**
 * Desktop navigation links — page buttons + Explore dropdown.
 *
 * Order: Create (flag-gated) | Explore | Pricing | FAQ
 * Hidden below the `lg` breakpoint via the parent's `hidden lg:flex`.
 * Extracted from Navbar for single-responsibility.
 */

import { useFeatureFlag } from "@/lib/useFeatureFlag";

import { NavButton } from "./NavButton";
import { RecipesMenu } from "./RecipesMenu";
import { PAGE_LINKS } from "./navData";

export function DesktopNav() {
  const showEditor = useFeatureFlag("editor");

  return (
    <div className="hidden items-center gap-2 lg:flex">
      {showEditor && <NavButton href="/create">Create</NavButton>}
      <RecipesMenu />
      {PAGE_LINKS.map((link) => (
        <NavButton key={link.href} href={link.href}>
          {link.label}
        </NavButton>
      ))}
    </div>
  );
}
