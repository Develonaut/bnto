"use client";

import { Fragment, useCallback, useMemo, useState } from "react";
import {
  Badge,
  Dialog,
  DialogClose,
  DialogContent,
  DialogHeader,
  DialogTitle,
  Divider,
  SearchIcon,
  Text,
} from "@bnto/ui";
import { useEditor } from "../../context";
import { useNodePalette } from "../../hooks/useNodePalette";
import type { PaletteItem } from "../../hooks/useNodePalette";
import { SLOTS } from "../../adapters/bentoSlots";

/**
 * NodePaletteDialog — dialog-based node palette.
 *
 * Shows a searchable categorized grid of palette items. Multi-operation
 * node types (image, spreadsheet, file-system) appear as separate entries
 * with the operation pre-set. Clicking an item adds the node to the canvas
 * and closes the dialog.
 */

interface NodePaletteDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

/** Unique key for a palette item — disambiguates split operations. */
function itemKey(item: PaletteItem): string {
  const op = item.defaultParams?.operation;
  return op ? `${item.type}:${op}` : item.type;
}

function NodePaletteDialogRoot({ open, onOpenChange }: NodePaletteDialogProps) {
  const [search, setSearch] = useState("");
  const editor = useEditor();
  const { groups } = useNodePalette();
  const { nodes, insertAfterNodeId, insertIntoContainerId } = editor.nodes.useNodes();
  const nodeCount = nodes.length;
  const isFull = nodeCount >= SLOTS.length;

  const filteredGroups = useMemo(() => {
    const term = search.trim().toLowerCase();
    if (!term) return groups;
    return groups
      .map((group) => ({
        ...group,
        items: group.items.filter(
          (item) =>
            item.type.toLowerCase().includes(term) ||
            item.label.toLowerCase().includes(term) ||
            item.description.toLowerCase().includes(term),
        ),
      }))
      .filter((group) => group.items.length > 0);
  }, [groups, search]);

  const handleClose = useCallback(
    (nextOpen: boolean) => {
      if (!nextOpen) {
        editor.nodes.setInsertAfterNodeId(null);
        editor.nodes.setInsertIntoContainerId(null);
      }
      onOpenChange(nextOpen);
    },
    [editor, onOpenChange],
  );

  const handleAdd = useCallback(
    (item: PaletteItem) => {
      editor.nodes.addNode(item.type, insertAfterNodeId, insertIntoContainerId, item.defaultParams);
      handleClose(false);
    },
    [editor, insertAfterNodeId, insertIntoContainerId, handleClose],
  );

  const handleAddItem = useCallback((item: PaletteItem) => () => handleAdd(item), [handleAdd]);

  const handleSearchChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => setSearch(e.target.value),
    [],
  );

  const hasResults = filteredGroups.length > 0;

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent size="lg" data-testid="node-palette-dialog">
        <DialogHeader className="pb-2">
          <DialogTitle>Add Node</DialogTitle>
          <DialogClose />
        </DialogHeader>

        <div className="flex h-[28rem] flex-col">
          {/* Search */}
          <div className="shrink-0 pb-3">
            <div className="flex items-center gap-2 rounded-md bg-input px-2.5 py-1.5">
              <SearchIcon className="size-3.5 shrink-0 text-muted-foreground" />
              <input
                type="text"
                placeholder="Search nodes..."
                aria-label="Search node types"
                value={search}
                onChange={handleSearchChange}
                className="w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
                autoFocus
              />
            </div>
          </div>

          {isFull && (
            <Text size="xs" color="muted" className="pb-2">
              Canvas is full ({SLOTS.length} nodes max).
            </Text>
          )}

          <Divider />

          {/* Node grid */}
          <div className="min-h-0 flex-1 overflow-y-auto pt-3">
            <div className="grid grid-cols-2 gap-1">
              {filteredGroups.map((group, i) => (
                <Fragment key={group.category.name}>
                  {i > 0 && <hr className="col-span-2 my-1 border-border" />}
                  <h3 className="col-span-2 px-2 pt-1 pb-0.5 text-xs font-medium text-muted-foreground">
                    {group.category.label}
                  </h3>
                  {group.items.map((item) => {
                    const isServerOnly = !item.browserCapable;
                    const disabled = isFull || isServerOnly;
                    return (
                      <button
                        key={itemKey(item)}
                        type="button"
                        onClick={handleAddItem(item)}
                        disabled={disabled}
                        data-testid={`palette-item-${itemKey(item)}`}
                        className="flex flex-col items-start gap-1 rounded-md px-2 py-2.5 text-left transition-colors hover:bg-muted disabled:pointer-events-none disabled:opacity-50"
                      >
                        <span className="flex w-full items-center gap-2">
                          <span className="text-sm leading-normal font-medium">{item.label}</span>
                          {isServerOnly && (
                            <Badge variant="outline" className="shrink-0 text-xs">
                              Pro
                            </Badge>
                          )}
                        </span>
                        <span className="text-xs leading-normal text-muted-foreground text-wrap">
                          {item.description}
                        </span>
                      </button>
                    );
                  })}
                </Fragment>
              ))}

              {!hasResults && search.trim() && (
                <Text size="xs" color="muted" className="col-span-2 px-3 py-4 text-center">
                  No nodes match &ldquo;{search}&rdquo;
                </Text>
              )}
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}

export { NodePaletteDialogRoot };
