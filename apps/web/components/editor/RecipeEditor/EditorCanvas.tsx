"use client";

import { RECIPES } from "@bnto/nodes";
import { Select } from "@/components/ui/Select";
import { BentoCanvas } from "../canvas/BentoCanvas";
import { EditorSidebar } from "../EditorSidebar";
import { CanvasToolbar } from "../CanvasToolbar";
import { NodeConfigPanel } from "../NodeConfigPanel";
import { EditorOverlay } from "./EditorOverlay";
import { useEditorCanvas } from "./useEditorCanvas";

/**
 * EditorCanvas — render shell for the recipe editor.
 *
 * All orchestration logic (selection, sync, recipe change, etc.)
 * lives in useEditorCanvas. This component is pure layout.
 */

interface EditorCanvasProps {
  initialSlug?: string;
}

function EditorCanvas({ initialSlug }: EditorCanvasProps) {
  const {
    sidebarCollapsed,
    toggleSidebar,
    selectedNodeId,
    configNodeId,
    recipeName,
    activeSlug,
    handleRecipeChange,
    handleReset,
    defaultNodes,
  } = useEditorCanvas({ initialSlug });

  return (
    <div
      className="relative h-full overflow-hidden"
      data-testid="recipe-editor"
    >
      {/* Canvas — full bleed, no children. Fills entire viewport. */}
      <BentoCanvas
        defaultNodes={defaultNodes}
        interactive
        disable={{ drag: true }}
        standalone
        className="h-full rounded-none border-0"
      />

      {/* Floating panel layer */}
      <EditorOverlay>
        {/* Left sidebar — full height of safe area */}
        <EditorOverlay.Sidebar>
          <EditorSidebar
            collapsed={sidebarCollapsed}
            onToggle={toggleSidebar}
            selectedNodeId={selectedNodeId}
            name={recipeName}
            footer={
              <Select value={activeSlug} onValueChange={handleRecipeChange}>
                <Select.Trigger size="sm" className="w-full">
                  <Select.Value placeholder="Select recipe..." />
                </Select.Trigger>
                <Select.Content>
                  <Select.Item value="blank">Blank Canvas</Select.Item>
                  <Select.Separator />
                  {RECIPES.map((recipe) => (
                    <Select.Item key={recipe.slug} value={recipe.slug}>
                      {recipe.name}
                    </Select.Item>
                  ))}
                </Select.Content>
              </Select>
            }
          />
        </EditorOverlay.Sidebar>

        {/* Right config panel — slides in when node selected */}
        <EditorOverlay.ConfigPanel visible={!!selectedNodeId}>
          <NodeConfigPanel selectedNodeId={configNodeId} />
        </EditorOverlay.ConfigPanel>

        {/* Floating toolbar — bottom-center */}
        <EditorOverlay.Toolbar>
          <CanvasToolbar onReset={handleReset} />
        </EditorOverlay.Toolbar>
      </EditorOverlay>
    </div>
  );
}

export { EditorCanvas };
