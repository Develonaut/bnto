"use client";

import { useCallback, useState } from "react";
import {
  Plus,
  Trash2,
  Download,
  Undo2,
  Redo2,
  RotateCcw,
} from "lucide-react";
import { RECIPES } from "@bnto/nodes";
import { Button } from "@/components/ui/Button";
import { Row } from "@/components/ui/Row";
import { Select } from "@/components/ui/Select";
import { Text } from "@/components/ui/Text";
import { useEditorActions, useEditorStore } from "@/editor";
import { useEditorSelection } from "@/editor/hooks/useEditorSelection";
import { useEditorUndoRedo } from "@/editor/hooks/useEditorUndoRedo";
import { useEditorExport } from "@/editor/hooks/useEditorExport";

/**
 * EditorToolbar — action bar above the bento canvas.
 *
 * Recipe selector (all Tier 1 + "Blank"), Add Node (opens palette),
 * Remove Selected, Export, Undo/Redo, Reset. All state flows through
 * the editor store — the toolbar reads and dispatches, never owns state.
 */

interface EditorToolbarProps {
  /** Called when the user wants to open the node palette. */
  onOpenPalette?: () => void;
}

function EditorToolbar({ onOpenPalette }: EditorToolbarProps) {
  const { loadRecipe, createBlank, removeNode } = useEditorActions();
  const { selectedNodeId } = useEditorSelection();
  const { undo, redo, canUndo, canRedo } = useEditorUndoRedo();
  const { download, canExport } = useEditorExport();
  const isDirty = useEditorStore((s) => s.isDirty);
  const nodeCount = useEditorStore((s) => s.definition.nodes?.length ?? 0);

  /* Track the currently loaded recipe for the selector. "blank" = blank canvas. */
  const [activeSlug, setActiveSlug] = useState<string>("blank");

  const handleRecipeChange = useCallback(
    (slug: string) => {
      setActiveSlug(slug);
      if (slug === "blank") {
        createBlank();
      } else {
        loadRecipe(slug);
      }
    },
    [loadRecipe, createBlank],
  );

  const handleRemove = useCallback(() => {
    if (selectedNodeId) removeNode(selectedNodeId);
  }, [selectedNodeId, removeNode]);

  const handleReset = useCallback(() => {
    handleRecipeChange(activeSlug);
  }, [handleRecipeChange, activeSlug]);

  const handleExport = useCallback(() => {
    download();
  }, [download]);

  return (
    <Row className="flex-wrap items-center gap-2">
      {/* Recipe selector */}
      <Select value={activeSlug} onValueChange={handleRecipeChange}>
        <Select.Trigger size="sm" className="w-[180px]">
          <Select.Value placeholder="Select recipe…" />
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

      {/* Add Node */}
      <Button variant="secondary" onClick={onOpenPalette}>
        <Plus className="size-3.5" />
        Add Node
      </Button>

      {/* Remove Selected */}
      <Button
        variant="ghost"
        onClick={handleRemove}
        disabled={!selectedNodeId}
      >
        <Trash2 className="size-3.5" />
        Remove
      </Button>

      {/* Spacer */}
      <div className="flex-1" />

      {/* Undo / Redo */}
      <Button
        size="icon"
        variant="ghost"
        onClick={undo}
        disabled={!canUndo}
        aria-label="Undo"
      >
        <Undo2 className="size-3.5" />
      </Button>
      <Button
        size="icon"
        variant="ghost"
        onClick={redo}
        disabled={!canRedo}
        aria-label="Redo"
      >
        <Redo2 className="size-3.5" />
      </Button>

      {/* Reset */}
      <Button
        size="icon"
        variant="ghost"
        onClick={handleReset}
        disabled={!isDirty}
        aria-label="Reset"
      >
        <RotateCcw className="size-3.5" />
      </Button>

      {/* Export */}
      <Button
        variant="secondary"
        onClick={handleExport}
        disabled={!canExport || nodeCount === 0}
      >
        <Download className="size-3.5" />
        Export
      </Button>

      {/* Node count */}
      <Text size="xs" color="muted" className="font-mono">
        {nodeCount} nodes
      </Text>
    </Row>
  );
}

export { EditorToolbar };
