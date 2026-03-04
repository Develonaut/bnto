"use client";

import { useState, useCallback, useEffect, useMemo, useRef } from "react";
import {
  ReactFlowProvider,
  Panel as CanvasPanel,
  useReactFlow,
  useStoreApi,
  applyNodeChanges,
  type OnNodesChange,
} from "@xyflow/react";
import { getRecipeBySlug, RECIPES } from "@bnto/nodes";
import { cn } from "@/lib/cn";
import { Card } from "@/components/ui/Card";
import { Select } from "@/components/ui/Select";
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
import { CanvasToolbar } from "./CanvasToolbar";
import { NodePalette } from "./NodePalette";
import { NodeConfigPanel } from "./NodeConfigPanel";

/**
 * RecipeEditor — canvas-first editor with floating panels.
 *
 * The canvas fills the entire container. Sidebar (left) and config
 * panel (right) float on top as elevated Card surfaces. Toolbar and
 * recipe selector float via ReactFlow `<Panel>` components.
 *
 * Two entry modes:
 *   <RecipeEditor slug="compress-images" />  — loads predefined recipe
 *   <RecipeEditor />                         — blank canvas
 */

interface RecipeEditorProps {
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
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [activeSlug, setActiveSlug] = useState<string>("blank");

  const { selectedNodeId } = useEditorSelection();
  const { setPositionGetter, loadRecipe, createBlank } = useEditorActions();
  const definition = useEditorStore((s) => s.definition);
  const { setNodes } = useReactFlow<CompartmentNodeType>();
  const storeApi = useStoreApi<CompartmentNodeType>();

  /* Register position getter so the store can read RF positions for undo. */
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

  const nodes = useCanvasNodes();

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

  const handleReset = useCallback(() => {
    handleRecipeChange(activeSlug);
  }, [handleRecipeChange, activeSlug]);

  const toggleSidebar = useCallback(() => {
    setSidebarCollapsed((prev) => !prev);
  }, []);

  return (
    <>
      <div
        className="relative h-[600px] overflow-hidden rounded-xl border border-border"
        data-testid="recipe-editor"
      >
        {/* Canvas fills the entire container */}
        <BentoCanvas
          nodes={nodes}
          height={600}
          interactive
          standalone
          onNodesChange={onNodesChange}
          className="rounded-none border-0"
        >
          {/* Recipe selector — top-left floating panel */}
          <CanvasPanel position="top-left" className="m-2">
            <Card elevation="md" className="p-1">
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
            </Card>
          </CanvasPanel>

          {/* Floating toolbar — bottom-center */}
          <CanvasPanel position="bottom-center" className="mb-3">
            <CanvasToolbar
              onOpenPalette={() => setPaletteOpen(true)}
              onReset={handleReset}
            />
          </CanvasPanel>
        </BentoCanvas>

        {/* Left floating sidebar — node list */}
        <div className="pointer-events-auto absolute bottom-2 left-2 top-14 z-10">
          <EditorSidebar
            collapsed={sidebarCollapsed}
            onToggle={toggleSidebar}
            selectedNodeId={selectedNodeId}
          />
        </div>

        {/* Right floating config panel — slides in when node selected.
            CSS property transition (not entrance animation) — correct per
            animation decision tree for transform/opacity state changes. */}
        <div
          className={cn(
            "pointer-events-auto absolute bottom-2 right-2 top-2 z-10 w-72 motion-safe:transition-[transform,opacity] motion-safe:duration-slow motion-safe:ease-spring-bouncy",
            selectedNodeId
              ? "translate-x-0 opacity-100"
              : "pointer-events-none translate-x-[110%] opacity-0",
          )}
        >
          <NodeConfigPanel selectedNodeId={selectedNodeId} />
        </div>
      </div>

      {/* Node palette (slide-out sheet) */}
      <NodePalette open={paletteOpen} onOpenChange={setPaletteOpen} />
    </>
  );
}

export { RecipeEditor };
