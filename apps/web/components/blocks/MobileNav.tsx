"use client";

/**
 * Mobile navigation island — owns the hamburger trigger, open/close state,
 * resize listener, and the MobileNavMenu sheet.
 *
 * Extracted so the parent Navbar can remain a server component.
 * Only rendered below the `lg` breakpoint (hidden via CSS).
 */

import { useEffect, useState } from "react";

import { Button, MenuIcon } from "@bnto/ui";

import { MobileNavMenu } from "./MobileNavMenu";

/**
 * Matches Tailwind's `lg` breakpoint (1024px).
 * Closes the mobile sheet when the viewport crosses into desktop territory.
 */
const LG_BREAKPOINT = 1024;

export function MobileNav() {
  const [open, setOpen] = useState(false);

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth > LG_BREAKPOINT) {
        setOpen(false);
      }
    };

    handleResize();
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  useEffect(() => {
    document.body.style.overflow = open ? "hidden" : "auto";
  }, [open]);

  return (
    <>
      <div className="lg:hidden">
        <Button variant="secondary" size="icon" elevation="sm" onClick={() => setOpen(!open)}>
          <MenuIcon />
          <span className="sr-only">Open menu</span>
        </Button>
      </div>
      <MobileNavMenu open={open} onOpenChange={setOpen} />
    </>
  );
}
