/**
 * Desktop sidebar — persistent left panel with 3D Card surface.
 *
 * Uses SidebarShell for the shared frame (Card, padding, logo spacing,
 * footer separator). Content slots filled with app-specific nav.
 * Hidden below `lg` breakpoint.
 */

import { cn, SidebarShell, SIDEBAR_WIDTH } from "@bnto/ui";

import { SidebarFooter } from "./SidebarFooter";
import { SidebarLogo } from "./SidebarLogo";
import { SidebarNav } from "./SidebarNav";

export function AppSidebar() {
  return (
    <aside className={cn("z-10 hidden h-dvh shrink-0 py-4 pl-4 pr-2 lg:flex", SIDEBAR_WIDTH)}>
      <SidebarShell header={<SidebarLogo />} footer={<SidebarFooter />}>
        <SidebarNav />
      </SidebarShell>
    </aside>
  );
}
