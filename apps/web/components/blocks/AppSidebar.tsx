/**
 * Desktop sidebar — persistent left panel with 3D Card surface.
 *
 * Hidden below `lg` breakpoint. The `p-2` on the aside creates
 * a gap around the Card so the 3D walls and shadow are visible.
 */

import { Card } from "@bnto/ui";

import { SidebarNav } from "./SidebarNav";

export function AppSidebar() {
  return (
    <aside className="z-10 hidden h-dvh w-72 shrink-0 py-4 pl-4 pr-2 lg:flex">
      <Card elevation="md" className="flex flex-1 flex-col">
        <div className="flex flex-1 flex-col overflow-y-auto p-4">
          <SidebarNav />
        </div>
      </Card>
    </aside>
  );
}
