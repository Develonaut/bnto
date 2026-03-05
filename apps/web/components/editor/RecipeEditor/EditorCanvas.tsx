"use client";

import { RECIPES } from "@bnto/nodes";
import { Select } from "@bnto/ui";
import { BentoCanvas } from "../canvas/BentoCanvas";
import { EditorPanel } from "../EditorPanel";
import { CanvasToolbar } from "../CanvasToolbar";
import { NodeConfigPanel } from "../NodeConfigPanel";
import { EditorOverlay } from "./EditorOverlay";
import { useEditorCanvas } from "./useEditorCanvas";

/**
 * EditorCanvas — render shell for the recipe editor.
 *
 * All orchestration logic lives in useEditorCanvas.
 * This component is pure layout.
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
    nodes,
    edges,
    onNodesChange,
    onEdgesChange,
  } = useEditorCanvas({ initialSlug });

  return (
    <div
      className="relative h-full overflow-hidden"
      data-testid="recipe-editor"
    >
      {/* Canvas — controlled mode, store owns state. */}
      <BentoCanvas
        nodes={nodes}
        onNodesChange={onNodesChange}
        edges={edges}
        onEdgesChange={onEdgesChange}
        interactive
        disable={{ drag: true }}
        standalone
        className="h-full rounded-none border-0"
      />

      {/* Floating panel layer */}
      <EditorOverlay>
        <EditorOverlay.Sidebar>
          <EditorPanel
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

        <EditorOverlay.ConfigPanel visible={!!selectedNodeId}>
          <NodeConfigPanel selectedNodeId={configNodeId} />
        </EditorOverlay.ConfigPanel>

        <EditorOverlay.Toolbar>
          <CanvasToolbar onReset={handleReset} />
        </EditorOverlay.Toolbar>
      </EditorOverlay>
    </div>
  );
}

export { EditorCanvas };
