"use client";

import { useCallback, useEffect, useState } from "react";

/**
 * Matches Tailwind's `lg` breakpoint (1024px).
 * Closes the mobile sheet when the viewport crosses into desktop territory.
 */
const LG_BREAKPOINT = 1024;

export function useMobileNav() {
  const [open, setOpen] = useState(false);

  const handleToggle = useCallback(() => setOpen((prev) => !prev), []);

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth > LG_BREAKPOINT) setOpen(false);
    };
    handleResize();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  useEffect(() => {
    document.body.style.overflow = open ? "hidden" : "auto";
  }, [open]);

  return { open, setOpen, handleToggle };
}
