"use client";

/**
 * Mobile navigation island — owns the hamburger trigger, open/close state,
 * resize listener, and the MobileNavMenu sheet.
 *
 * Extracted so the parent Navbar can remain a server component.
 * Only rendered below the `lg` breakpoint (hidden via CSS).
 */

import { Button, MenuIcon } from "@bnto/ui";

import { MobileNavMenu } from "./MobileNavMenu";
import { useMobileNav } from "./useMobileNav";

export function MobileNav() {
  const { open, setOpen, handleToggle } = useMobileNav();

  return (
    <>
      <div className="lg:hidden">
        <Button
          variant="secondary"
          size="icon"
          elevation="sm"
          onClick={handleToggle}
          data-testid="mobile-menu-button"
        >
          <MenuIcon />
          <span className="sr-only">Open menu</span>
        </Button>
      </div>
      <MobileNavMenu open={open} onOpenChange={setOpen} />
    </>
  );
}
