"use client";

import type { ReactNode } from "react";
import {
  cn,
  SidebarShell,
  SIDEBAR_WIDTH,
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@bnto/ui";
import { RecipeFileMenu } from "./RecipeFileMenu";
import { NodeListTab } from "./NodeListTab";
import { PaletteTab } from "./PaletteTab";

/**
 * EditorLeftPanel — always-visible side panel in the editor.
 *
 * Uses SidebarShell for the shared Card frame (same spacing as
 * AppSidebar). Header and footer are app-level slots composed from
 * apps/web. Content: recipe file menu + Nodes/Palette tabs.
 */

interface EditorLeftPanelProps {
  /** Logo — composed from apps/web to share with AppSidebar. */
  header?: ReactNode;
  /** App-level footer content (theme toggle, user menu). */
  footer?: ReactNode;
}

function EditorLeftPanelRoot({ header, footer }: EditorLeftPanelProps) {
  return (
    <div
      className={cn(
        "pointer-events-auto absolute -bottom-4 -left-4 -top-4 flex py-4 pl-4 pr-2",
        SIDEBAR_WIDTH,
      )}
    >
      <SidebarShell header={header} footer={footer}>
        <div className="flex h-full flex-col gap-3">
          <RecipeFileMenu />

          <Tabs defaultValue="nodes" className="flex min-h-0 flex-1 flex-col">
            <TabsList className="w-full shrink-0">
              <TabsTrigger value="nodes" className="flex-1" data-testid="panel-tab-nodes">
                Nodes
              </TabsTrigger>
              <TabsTrigger value="palette" className="flex-1" data-testid="panel-tab-palette">
                Palette
              </TabsTrigger>
            </TabsList>

            <TabsContent value="nodes" className="mt-0 min-h-0 flex-1">
              <NodeListTab />
            </TabsContent>
            <TabsContent value="palette" className="mt-0 min-h-0 flex-1">
              <PaletteTab />
            </TabsContent>
          </Tabs>
        </div>
      </SidebarShell>
    </div>
  );
}

export { EditorLeftPanelRoot };
export type { EditorLeftPanelProps };
