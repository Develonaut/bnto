"use client";

import { useState, useCallback, useEffect, useMemo, useRef } from "react";
import {
  ReactFlowProvider,
  useReactFlow,
  useStoreApi,
  applyNodeChanges,
  type OnNodesChange,
} from "@xyflow/react";
import { getRecipeBySlug } from "@bnto/nodes";
import { Stack } from "@/components/ui/Stack";
import {
  EditorProvider,
  useEditorStore,
  useEditorActions,
  definitionToBento,
} from "@/editor";
import { useCanvasNodes } from "@/editor/hooks/useCanvasNodes";
import { useDefinitionSync } from "@/editor/hooks/useDefinitionSync";
import { useEditorSelection } from "@/editor/hooks/useEditorSelection";
import { BentoCanvas } from "./canvas/BentoCanvas";
import type { CompartmentNodeType } from "./canvas/CompartmentNode";
import { EditorSidebar } from "./EditorSidebar";
import { EditorToolbar } from "./EditorToolbar";
import { NodePalette } from "./NodePalette";
import { NodeConfigPanel } from "./NodeConfigPanel";

/**
 * RecipeEditor — composes EditorToolbar + BentoCanvas + NodeConfigPanel.
 *
 * Two entry modes:
 *   <RecipeEditor slug="compress-images" />  — loads predefined recipe
 *   <RecipeEditor />                         — blank canvas
 *
 * Wraps children in EditorProvider (Zustand) + ReactFlowProvider (RF).
 * The inner component handles wiring between the editor store and the
 * canvas. All editor hooks share the same ReactFlowProvider context —
 * BentoCanvas runs in `standalone` mode (no internal provider).
 */

interface RecipeEditorProps {
  /** Predefined recipe slug to load. Omit for blank canvas. */
  slug?: string;
}

function RecipeEditor({ slug }: RecipeEditorProps) {
  const initialDefinition = useMemo(() => {
    if (!slug) return undefined;
    return getRecipeBySlug(slug)?.definition;
  }, [slug]);

  return (
    <EditorProvider initialDefinition={initialDefinition}>
      <ReactFlowProvider>
        <RecipeEditorInner />
      </ReactFlowProvider>
    </EditorProvider>
  );
}

/* ── Inner component — needs both provider contexts ──────────── */

function RecipeEditorInner() {
  const [paletteOpen, setPaletteOpen] = useState(false);
  const { selectedNodeId } = useEditorSelection();
  const { setPositionGetter } = useEditorActions();
  const definition = useEditorStore((s) => s.definition);
  const { setNodes } = useReactFlow<CompartmentNodeType>();
  const storeApi = useStoreApi<CompartmentNodeType>();

  /* Register position getter so the store can read RF positions for undo
   * snapshots. Uses storeApi (non-reactive) to avoid re-render loops. */
  useEffect(() => {
    setPositionGetter(() => {
      const positions: Record<string, { x: number; y: number }> = {};
      for (const node of storeApi.getState().nodes) {
        positions[node.id] = node.position;
      }
      return positions;
    });
  }, [setPositionGetter, storeApi]);

  /* Seed the RF canvas with bento nodes from the initial definition. */
  const mountedRef = useRef(false);
  useEffect(() => {
    if (mountedRef.current) return;
    mountedRef.current = true;
    const layout = definitionToBento(definition);
    setNodes(layout.nodes as CompartmentNodeType[]);
  }, [definition, setNodes]);

  /* Incremental sync: definition changes → RF node additions/removals. */
  useDefinitionSync();

  /* Full replacement sync: when the definition reference changes entirely
   * (loadRecipe, undo/redo), re-convert the full definition to bento. */
  const prevDefRef = useRef(definition);
  useEffect(() => {
    if (prevDefRef.current === definition) return;
    prevDefRef.current = definition;
    const layout = definitionToBento(definition);
    setNodes(layout.nodes as CompartmentNodeType[]);
  }, [definition, setNodes]);

  /* Controlled mode: apply RF visual changes (drag, select). */
  const onNodesChange: OnNodesChange<CompartmentNodeType> = useCallback(
    (changes) => {
      setNodes((prev) => applyNodeChanges(changes, prev));
    },
    [setNodes],
  );

  /* Read current RF nodes for BentoCanvas. */
  const nodes = useCanvasNodes();

  return (
    <Stack gap="sm" data-testid="recipe-editor">
      {/* Toolbar */}
      <EditorToolbar onOpenPalette={() => setPaletteOpen(true)} />

      {/* Canvas + Config panel side-by-side */}
      <div className="flex gap-4">
        {/* Canvas — takes remaining space. standalone=true to share
         * ReactFlowProvider with sibling hooks. */}
        <div className="relative flex-1 min-w-0">
          <BentoCanvas
            nodes={nodes}
            height={520}
            interactive
            standalone
            onNodesChange={onNodesChange}
          />
          {/* Sidebar overlays the canvas from top-left, full height */}
          <div className="absolute inset-y-0 left-0 z-10 p-2">
            <EditorSidebar selectedNodeId={selectedNodeId} />
          </div>
        </div>

        {/* Config panel — fixed width on the right */}
        <div className="w-72 shrink-0">
          <NodeConfigPanel selectedNodeId={selectedNodeId} />
        </div>
      </div>

      {/* Node palette (slide-out sheet) */}
      <NodePalette open={paletteOpen} onOpenChange={setPaletteOpen} />
    </Stack>
  );
}

export { RecipeEditor };
