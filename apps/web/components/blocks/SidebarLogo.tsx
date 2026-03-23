/**
 * SidebarLogo — bnto wordmark link, shared by app sidebar and editor panel.
 *
 * Uses NavButton for the optimistic pressed state on navigation.
 */

"use client";

import { NavButton } from "./NavButton";

export function SidebarLogo() {
  return (
    <NavButton
      href="/"
      className="w-fit text-xl font-display font-black tracking-tighter"
      data-testid="nav-link-home"
    >
      bnto
    </NavButton>
  );
}
