"use client";

import { useCallback } from "react";
import { Button } from "@/components/ui/Button";
import { Toolbar } from "@/components/ui/Toolbar";
import { Text } from "@/components/ui/Text";
import {
  PlusIcon,
  TrashIcon,
  PlayIcon,
  RotateCcwIcon,
  Undo2Icon,
  Redo2Icon,
} from "@/components/ui/icons";
import { useEditorStore } from "@/editor/hooks/useEditorStore";
import { useEditorSelection } from "@/editor/hooks/useEditorSelection";
import { useEditorUndoRedo } from "@/editor/hooks/useEditorUndoRedo";
import { useRemoveNode } from "@/editor/hooks/useRemoveNode";
import { NodePaletteMenu } from "./NodePaletteMenu";

/**
 * CanvasToolbar — floating toolbar rendered inside a ReactFlow
 * `<Panel position="bottom-center">`.
 *
 * Contains: Add Node (dropdown menu), Remove Selected, Undo/Redo,
 * Reset, Run. Export lives in the sidebar file menu.
 */

interface CanvasToolbarProps {
  onReset?: () => void;
  onRun?: () => void;
}

function CanvasToolbar({ onReset, onRun }: CanvasToolbarProps) {
  const removeNode = useRemoveNode();
  const { selectedNodeId } = useEditorSelection();
  const { undo, redo, canUndo, canRedo } = useEditorUndoRedo();
  const isDirty = useEditorStore((s) => s.isDirty);
  const nodeCount = useEditorStore((s) => s.definition.nodes?.length ?? 0);

  const handleRemove = useCallback(() => {
    if (selectedNodeId) removeNode(selectedNodeId);
  }, [selectedNodeId, removeNode]);

  return (
    <Toolbar elevation="md" className="pointer-events-auto">
      {/* Add (dropdown) / Remove */}
      <Toolbar.Group>
        <NodePaletteMenu>
          <NodePaletteMenu.Trigger variant="ghost" elevation="sm">
            <PlusIcon className="size-4" />
            Add
          </NodePaletteMenu.Trigger>
          <NodePaletteMenu.Content side="top" offset="lg" />
        </NodePaletteMenu>
        <Button
          size="icon"
          variant="ghost"
          elevation="sm"
          onClick={handleRemove}
          disabled={!selectedNodeId}
          aria-label="Remove selected node"
        >
          <TrashIcon className="size-4" />
        </Button>
      </Toolbar.Group>

      <Toolbar.Divider />

      {/* Undo / Redo / Reset */}
      <Toolbar.Group>
        <Button
          size="icon"
          variant="ghost"
          elevation="sm"
          onClick={undo}
          disabled={!canUndo}
          aria-label="Undo"
        >
          <Undo2Icon className="size-4" />
        </Button>
        <Button
          size="icon"
          variant="ghost"
          elevation="sm"
          onClick={redo}
          disabled={!canRedo}
          aria-label="Redo"
        >
          <Redo2Icon className="size-4" />
        </Button>
        <Button
          size="icon"
          variant="ghost"
          elevation="sm"
          onClick={onReset}
          disabled={!isDirty}
          aria-label="Reset"
        >
          <RotateCcwIcon className="size-4" />
        </Button>
      </Toolbar.Group>

      <Toolbar.Divider />

      {/* Run + count */}
      <Toolbar.Group>
        <Button
          variant="ghost"
          elevation="sm"
          onClick={onRun}
          disabled={nodeCount === 0}
        >
          <PlayIcon className="size-4" />
          Run
        </Button>
        <Text size="xs" color="muted" className="font-mono">
          {nodeCount}
        </Text>
      </Toolbar.Group>
    </Toolbar>
  );
}

export { CanvasToolbar };
