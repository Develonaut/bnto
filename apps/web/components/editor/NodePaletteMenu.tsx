"use client";

import { Fragment, useCallback, useMemo, useState, type ComponentProps } from "react";
import type { NodeTypeName } from "@bnto/nodes";
import { Badge } from "@/components/ui/Badge";
import { Menu } from "@/components/ui/Menu";
import { Text } from "@/components/ui/Text";
import { SearchIcon } from "@/components/ui/icons";
import { useEditorStore } from "@/editor";
import { useNodePalette } from "@/editor/hooks/useNodePalette";
import { useAddNode } from "@/editor/hooks/useAddNode";
import { SLOTS } from "@/editor/adapters/bentoSlots";

/**
 * NodePaletteMenu — composable Menu for adding nodes to the canvas.
 *
 * Compound component (dot-notation) that wraps the generic Menu with
 * editor-specific node palette content. Search filter at the top,
 * categorized node list below. Clicking adds the node and selects it
 * so the config panel opens immediately.
 *
 *   <NodePaletteMenu>
 *     <NodePaletteMenu.Trigger variant="ghost">
 *       <PlusIcon /> Add
 *     </NodePaletteMenu.Trigger>
 *     <NodePaletteMenu.Content side="top" />
 *   </NodePaletteMenu>
 */

/* ── Root — delegates to Menu ──────────────────────────────────── */

function NodePaletteMenuRoot({ children, ...props }: ComponentProps<typeof Menu>) {
  return <Menu {...props}>{children}</Menu>;
}

/* ── Trigger — pass-through to Menu.Trigger ────────────────────── */

const NodePaletteMenuTrigger = Menu.Trigger;

/* ── Content — search header + categorized node type list ──────── */

function NodePaletteMenuContent({
  className,
  ...props
}: Omit<ComponentProps<typeof Menu.Content>, "children">) {
  const [search, setSearch] = useState("");
  const addNode = useAddNode();
  const { groups } = useNodePalette();
  const nodeCount = useEditorStore((s) => s.definition.nodes?.length ?? 0);
  const isFull = nodeCount >= SLOTS.length;

  /* Filter groups by search term. */
  const filteredGroups = useMemo(() => {
    const term = search.trim().toLowerCase();
    if (!term) return groups;
    return groups
      .map((group) => ({
        ...group,
        items: group.items.filter(
          (item) =>
            item.label.toLowerCase().includes(term) ||
            item.description.toLowerCase().includes(term) ||
            item.name.toLowerCase().includes(term),
        ),
      }))
      .filter((group) => group.items.length > 0);
  }, [groups, search]);

  /* Just add the node — the sidebar effect handles selection + fitView. */
  const handleAdd = useCallback(
    (typeName: NodeTypeName) => {
      addNode(typeName);
    },
    [addNode],
  );

  const hasResults = filteredGroups.length > 0;

  return (
    <Menu.Content className={className ?? "w-[28rem] p-2"} {...props}>
      {/* Search header — sticky, doesn't close menu on click */}
      <div className="px-1 pb-2">
        <div className="flex items-center gap-2 rounded-md bg-input px-2.5 py-1.5">
          <SearchIcon className="size-3.5 shrink-0 text-muted-foreground" />
          <input
            type="text"
            placeholder="Search nodes…"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
            autoFocus
          />
        </div>
      </div>

      {isFull && (
        <Text size="xs" color="muted" className="px-3 py-2">
          Canvas is full ({SLOTS.length} nodes max).
        </Text>
      )}

      {/* Scrollable multi-column node list */}
      <div className="max-h-72 overflow-y-auto">
        <div className="grid grid-cols-2 gap-1">
          {filteredGroups.map((group, i) => (
            <Fragment key={group.category.name}>
              {i > 0 && <Menu.Separator className="col-span-2" />}
              <Menu.Label className="col-span-2">
                {group.category.label}
              </Menu.Label>
              {group.items.map((item) => {
                const isServerOnly = !item.browserCapable;
                const disabled = isFull || isServerOnly;
                return (
                  <Menu.Item
                    key={item.name}
                    onClick={() => handleAdd(item.name as NodeTypeName)}
                    disabled={disabled}
                    className="flex-col items-start gap-1 py-2.5 text-left"
                  >
                    <span className="flex w-full items-center gap-2">
                      <span className="text-sm leading-normal font-medium">
                        {item.label}
                      </span>
                      {isServerOnly && (
                        <Badge variant="outline" className="shrink-0 text-xs">
                          Pro
                        </Badge>
                      )}
                    </span>
                    <span className="text-xs leading-normal text-muted-foreground text-wrap">
                      {item.description}
                    </span>
                  </Menu.Item>
                );
              })}
            </Fragment>
          ))}

          {/* No results */}
          {!hasResults && search.trim() && (
            <Text size="xs" color="muted" className="col-span-2 px-3 py-4 text-center">
              No nodes match &ldquo;{search}&rdquo;
            </Text>
          )}
        </div>
      </div>
    </Menu.Content>
  );
}

export const NodePaletteMenu = Object.assign(NodePaletteMenuRoot, {
  Root: NodePaletteMenuRoot,
  Trigger: NodePaletteMenuTrigger,
  Content: NodePaletteMenuContent,
});
