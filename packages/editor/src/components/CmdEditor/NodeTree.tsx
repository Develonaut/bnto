"use client";

import { useMemo } from "react";
import { Text } from "@bnto/ui";
import { useEditor } from "../../context";
import { buildNodeListTree } from "../../helpers/buildNodeListTree";
import type { NodeListEntry } from "../../helpers/buildNodeListTree";

/**
 * NodeTree — minimal text display of the recipe node list.
 *
 * Read-only overview. All interaction goes through the command input.
 */
function NodeTree() {
  const editor = useEditor();
  const { nodes, configs, selectedNodeId, expandedContainerIds } = editor.nodes.useNodes();

  const tree = useMemo(() => buildNodeListTree(nodes, configs), [nodes, configs]);

  if (tree.entries.length === 0) {
    return (
      <Text size="sm" color="muted" className="py-4 text-center">
        Empty recipe. Type /add to get started.
      </Text>
    );
  }

  return (
    <div className="flex flex-col gap-0.5" role="tree" aria-label="Recipe nodes">
      {tree.entries.map((entry, i) => (
        <div key={entry.node.id}>
          {tree.placeholderIndex === i && <PlaceholderRow />}
          <EntryRows
            entry={entry}
            selectedNodeId={selectedNodeId}
            expandedIds={expandedContainerIds}
            depth={0}
          />
        </div>
      ))}
      {tree.placeholderIndex === tree.entries.length && <PlaceholderRow />}
    </div>
  );
}

/** Recursively renders a node and its expanded children. */
function EntryRows({
  entry,
  selectedNodeId,
  expandedIds,
  depth,
}: {
  entry: NodeListEntry;
  selectedNodeId: string | null;
  expandedIds: Set<string>;
  depth: number;
}) {
  return (
    <>
      <NodeRow entry={entry} selected={selectedNodeId === entry.node.id} depth={depth} />
      {entry.isContainer &&
        expandedIds.has(entry.node.id) &&
        entry.children.map((child) => (
          <EntryRows
            key={child.node.id}
            entry={child}
            selectedNodeId={selectedNodeId}
            expandedIds={expandedIds}
            depth={depth + 1}
          />
        ))}
    </>
  );
}

function NodeRow({
  entry,
  selected,
  depth,
}: {
  entry: NodeListEntry;
  selected: boolean;
  depth: number;
}) {
  const editor = useEditor();
  const label = entry.config.displayName ?? entry.config.name;
  const isIo = entry.node.data.isIoNode ?? false;

  return (
    <button
      type="button"
      className={`flex w-full items-center gap-1 rounded-md px-2 py-1 text-left text-sm outline-none ${
        selected ? "bg-accent/40 font-medium" : "hover:bg-muted/50"
      }`}
      style={depth > 0 ? { paddingLeft: `${depth * 16 + 8}px` } : undefined}
      role="treeitem"
      aria-selected={selected}
      onClick={() => editor.nodes.selectNode(entry.node.id)}
    >
      <span className="w-3 shrink-0 text-muted-foreground">{selected ? ">" : ""}</span>
      <span className={isIo ? "text-muted-foreground" : ""}>{label}</span>
    </button>
  );
}

function PlaceholderRow() {
  return <div className="px-2 py-1 text-sm text-muted-foreground">+ add a node</div>;
}

export { NodeTree };
