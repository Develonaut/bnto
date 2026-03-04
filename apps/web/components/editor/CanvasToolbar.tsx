"use client";

import { useCallback } from "react";
import { Button } from "@/components/ui/Button";
import { Toolbar } from "@/components/ui/Toolbar";
import { Text } from "@/components/ui/Text";
import {
  PlusIcon,
  TrashIcon,
  DownloadIcon,
  RotateCcwIcon,
  Undo2Icon,
  Redo2Icon,
} from "@/components/ui/icons";
import { useEditorActions } from "@/editor/hooks/useEditorActions";
import { useEditorStore } from "@/editor/hooks/useEditorStore";
import { useEditorSelection } from "@/editor/hooks/useEditorSelection";
import { useEditorUndoRedo } from "@/editor/hooks/useEditorUndoRedo";
import { useEditorExport } from "@/editor/hooks/useEditorExport";

/**
 * CanvasToolbar — floating toolbar rendered inside a ReactFlow
 * `<Panel position="bottom-center">`.
 *
 * Contains: Add Node, Remove Selected, Undo/Redo, Reset, Export.
 * Buttons follow the NavButton pattern (ghost + elevation="sm").
 */

interface CanvasToolbarProps {
  onOpenPalette?: () => void;
  onReset?: () => void;
}

function CanvasToolbar({ onOpenPalette, onReset }: CanvasToolbarProps) {
  const { removeNode } = useEditorActions();
  const { selectedNodeId } = useEditorSelection();
  const { undo, redo, canUndo, canRedo } = useEditorUndoRedo();
  const { download, canExport } = useEditorExport();
  const isDirty = useEditorStore((s) => s.isDirty);
  const nodeCount = useEditorStore((s) => s.definition.nodes?.length ?? 0);

  const handleRemove = useCallback(() => {
    if (selectedNodeId) removeNode(selectedNodeId);
  }, [selectedNodeId, removeNode]);

  const handleExport = useCallback(() => {
    download();
  }, [download]);

  return (
    <Toolbar elevation="md" className="pointer-events-auto">
      {/* Add / Remove */}
      <Toolbar.Group>
        <Button variant="ghost" elevation="sm" onClick={onOpenPalette}>
          <PlusIcon className="size-4" />
          Add
        </Button>
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

      {/* Export + count */}
      <Toolbar.Group>
        <Button
          variant="ghost"
          elevation="sm"
          onClick={handleExport}
          disabled={!canExport || nodeCount === 0}
        >
          <DownloadIcon className="size-4" />
          Export
        </Button>
        <Text size="xs" color="muted" className="font-mono">
          {nodeCount}
        </Text>
      </Toolbar.Group>
    </Toolbar>
  );
}

export { CanvasToolbar };
