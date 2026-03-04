"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { cn } from "@/lib/cn";
import { Menu } from "@/components/ui/Menu";
import { Text } from "@/components/ui/Text";
import { PanelLeftIcon, LayersIcon } from "@/components/ui/icons";
import { useCanvasNodes } from "@/editor/hooks/useCanvasNodes";
import type { CompartmentNodeType } from "./canvas/CompartmentNode";
import type { CompartmentVariant } from "@/editor/adapters/types";

/**
 * EditorSidebar — button that opens a left-side Menu listing canvas nodes.
 *
 * Reads nodes directly from the ReactFlow store (not Zustand editor).
 * Clicking a node updates RF selection via setNodes().
 */

/* ── Props ──────────────────────────────────────────────────── */

interface EditorSidebarProps {
  selectedNodeId: string | null;
}

/* ── Constants ──────────────────────────────────────────────── */

/** Variant → Tailwind bg class for the category dot. */
const DOT_COLOR: Record<CompartmentVariant, string> = {
  primary: "bg-primary",
  secondary: "bg-secondary",
  accent: "bg-accent",
  muted: "bg-muted-foreground",
  success: "bg-green-500",
  warning: "bg-amber-500",
};

/* ── Root ────────────────────────────────────────────────────── */

function EditorSidebarRoot({ selectedNodeId }: EditorSidebarProps) {
  const nodes = useCanvasNodes();
  const { setNodes } = useReactFlow<CompartmentNodeType>();

  const handleSelectNode = useCallback(
    (nodeId: string) => {
      setNodes((prev) =>
        prev.map((n) => ({ ...n, selected: n.id === nodeId })),
      );
    },
    [setNodes],
  );

  return (
    <Menu>
      <Menu.Trigger size="icon" variant="ghost" aria-label="Open node list">
        <PanelLeftIcon className="size-4" />
      </Menu.Trigger>
      <Menu.Content side="bottom" align="start" className="w-52">
        <PanelHeader count={nodes.length} />
        <NodeList
          nodes={nodes}
          selectedNodeId={selectedNodeId}
          onSelect={handleSelectNode}
        />
      </Menu.Content>
    </Menu>
  );
}

/* ── PanelHeader — title + count ─────────────────────────────── */

function PanelHeader({ count }: { count: number }) {
  return (
    <div className="flex items-center gap-2 px-3 py-2">
      <LayersIcon className="size-3.5 text-muted-foreground" />
      <Text size="sm" className="font-medium">
        Nodes
      </Text>
      <Text size="xs" color="muted" className="ml-auto font-mono">
        {count}
      </Text>
    </div>
  );
}

/* ── NodeList — scrollable list or empty prompt ──────────────── */

interface NodeListProps {
  nodes: CompartmentNodeType[];
  selectedNodeId: string | null;
  onSelect: (nodeId: string) => void;
}

function NodeList({ nodes, selectedNodeId, onSelect }: NodeListProps) {
  if (nodes.length === 0) {
    return (
      <div className="px-3 py-4">
        <Text size="xs" color="muted" className="text-center">
          No nodes yet. Add one from the toolbar.
        </Text>
      </div>
    );
  }

  return (
    <ul
      className="flex flex-col gap-0.5 p-1.5"
      role="listbox"
      aria-label="Canvas nodes"
    >
      {nodes.map((node) => (
        <NodeItem
          key={node.id}
          node={node}
          selected={node.id === selectedNodeId}
          onSelect={onSelect}
        />
      ))}
    </ul>
  );
}

/* ── NodeItem — single row in the list ───────────────────────── */

interface NodeItemProps {
  node: CompartmentNodeType;
  selected: boolean;
  onSelect: (nodeId: string) => void;
}

function NodeItem({ node, selected, onSelect }: NodeItemProps) {
  const variant = node.data.variant ?? "muted";

  return (
    <li role="option" aria-selected={selected}>
      <Menu.Close asChild>
        <button
          type="button"
          onClick={() => onSelect(node.id)}
          data-selected={selected}
          className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left transition-colors duration-fast hover:bg-muted data-[selected=true]:bg-muted data-[selected=true]:ring-2 data-[selected=true]:ring-ring"
        >
          <span
            className={cn("size-2 shrink-0 rounded-full", DOT_COLOR[variant])}
          />
          <span className="flex min-w-0 flex-1 flex-col">
            <Text size="sm" className="truncate font-medium leading-tight">
              {node.data.label}
            </Text>
            {node.data.sublabel && (
              <Text size="xs" color="muted" className="truncate leading-tight">
                {node.data.sublabel}
              </Text>
            )}
          </span>
        </button>
      </Menu.Close>
    </li>
  );
}

/* ── Public API ──────────────────────────────────────────────── */

export const EditorSidebar = Object.assign(EditorSidebarRoot, {
  Root: EditorSidebarRoot,
  PanelHeader,
  NodeList,
  NodeItem,
});
