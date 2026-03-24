"use client";

import { useState, useCallback, useMemo, useRef, useEffect } from "react";
import { useStore } from "zustand";
import {
  Command,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  Surface,
  Kbd,
  KbdGroup,
} from "@bnto/ui";
import { useEditor, useEditorStoreApi } from "../../context";
import { useCommands } from "../../hooks/useCommands";
import { buildNodeListTree } from "../../helpers/buildNodeListTree";
import { flattenTreeEntries } from "../../helpers/flattenTreeEntries";
import { resolveShiftArrow } from "../../actions/resolveShiftArrow";
import { CommandItemRow } from "./CommandItemRow";
import type {
  Command as CommandType,
  CommandGroup as CommandGroupType,
} from "../../commands/types";

// ---------------------------------------------------------------------------
// Navigation state — Notion-style drill-down
// ---------------------------------------------------------------------------

type NavLevel =
  | { kind: "root" }
  | { kind: "categories" }
  | { kind: "processors"; category: string };

// ---------------------------------------------------------------------------
// CmdInput
// ---------------------------------------------------------------------------

function CmdInput() {
  const editor = useEditor();
  const storeApi = useEditorStoreApi();
  const { rootGroups, categoryGroups, processorGroups } = useCommands();
  const [value, setValue] = useState("");
  const [nav, setNav] = useState<NavLevel>({ kind: "root" });
  const inputRef = useRef<HTMLInputElement>(null);

  // Refocus input after nav changes (drill-down / back).
  // The key prop on <Command> forces a remount for auto-selection,
  // so we need to re-grab focus on the new input element.
  useEffect(() => {
    inputRef.current?.focus();
  }, [nav]);

  const nodes = useStore(storeApi, (s) => s.nodes);
  const configs = useStore(storeApi, (s) => s.configs);
  const selectedNodeId = useStore(storeApi, (s) => s.selectedNodeId);
  const expandedContainerIds = useStore(storeApi, (s) => s.expandedContainerIds);

  const handleShiftArrow = useCallback(
    (key: string) => {
      const tree = buildNodeListTree(nodes, configs);
      const flatIds = flattenTreeEntries(tree.entries, expandedContainerIds);
      const result = resolveShiftArrow(key, flatIds, nodes, selectedNodeId, expandedContainerIds);
      if (!result) return;
      if (result.kind === "select") editor.nodes.selectNode(result.nodeId);
      else if (result.kind === "expand") editor.nodes.expandContainer(result.nodeId);
      else if (result.kind === "collapse") editor.nodes.collapseContainer(result.nodeId);
    },
    [nodes, configs, selectedNodeId, expandedContainerIds, editor],
  );

  const drillToCategories = useCallback(() => {
    setNav({ kind: "categories" });
    setValue("");
  }, []);

  const drillToProcessors = useCallback((category: string) => {
    setNav({ kind: "processors", category });
    setValue("");
  }, []);

  const goBack = useCallback(() => {
    setValue("");
    setNav((prev) => {
      if (prev.kind === "processors") return { kind: "categories" };
      if (prev.kind === "categories") return { kind: "root" };
      return prev;
    });
  }, []);

  const resetToRoot = useCallback(() => {
    setNav({ kind: "root" });
    setValue("");
  }, []);

  // Resolve the active command groups for the current nav level
  const groups: CommandGroupType[] = useMemo(() => {
    switch (nav.kind) {
      case "root":
        return rootGroups(drillToCategories);
      case "categories":
        return categoryGroups(drillToProcessors);
      case "processors":
        return processorGroups(nav.category);
    }
  }, [nav, rootGroups, categoryGroups, processorGroups, drillToCategories, drillToProcessors]);

  const handleSelect = useCallback(
    (cmd: CommandType) => {
      if (cmd.disabled) return;
      cmd.execute();
      // If the command drills down (add-node, cat-*), don't reset —
      // the drill-down callbacks already update nav state.
      // For terminal actions (undo, redo, add-X), reset to root.
      if (!cmd.id.startsWith("add-node") && !cmd.id.startsWith("cat-")) {
        resetToRoot();
      }
    },
    [resetToRoot],
  );

  const placeholder =
    nav.kind === "root"
      ? "Type a command..."
      : nav.kind === "categories"
        ? "Choose a category..."
        : "Choose a processor...";

  const showBack = nav.kind !== "root";

  return (
    <Command
      key={nav.kind === "processors" ? `processors-${nav.category}` : nav.kind}
      data-testid="cmd-input"
      className="overflow-visible"
      onKeyDown={(e) => {
        // Shift+Arrow — tree navigation from the command input
        if (e.shiftKey && ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.key)) {
          e.preventDefault();
          handleShiftArrow(e.key);
          return;
        }
        // Left arrow — go back a level (when input is empty)
        if (e.key === "ArrowLeft" && value === "" && showBack) {
          e.preventDefault();
          goBack();
          return;
        }
        // Right arrow — drill into the highlighted item if it's drillable
        if (e.key === "ArrowRight" && value === "") {
          const selected = (e.currentTarget as HTMLElement).querySelector<HTMLElement>(
            "[cmdk-item][data-selected=true]",
          );
          const testId = selected?.getAttribute("data-testid") ?? "";
          if (testId === "cmd-add-node" || testId.startsWith("cmd-cat-")) {
            e.preventDefault();
            selected?.click();
          }
          return;
        }
        // Backspace on empty input goes back a level
        if (e.key === "Backspace" && value === "" && showBack) {
          e.preventDefault();
          goBack();
        }
      }}
    >
      <CommandInput
        ref={inputRef}
        placeholder={placeholder}
        value={value}
        onValueChange={setValue}
      />
      <Surface className="mt-2 p-2">
        <CommandList>
          <CommandEmpty>No matching commands.</CommandEmpty>
          {groups.map((group) => (
            <CommandGroup key={group.group} heading={group.group}>
              {group.commands.map((cmd) => (
                <CommandItemRow
                  key={cmd.id}
                  command={cmd}
                  drillable={cmd.id === "add-node" || cmd.id.startsWith("cat-")}
                  onSelect={() => handleSelect(cmd)}
                />
              ))}
            </CommandGroup>
          ))}
        </CommandList>
        <div className="flex flex-wrap items-center gap-x-1.5 gap-y-1 border-t border-border px-3 py-2 text-xs text-muted-foreground">
          <KbdGroup>
            <Kbd>{"←"}</Kbd>
            <Kbd>{"→"}</Kbd>
          </KbdGroup>
          <span>navigate menus</span>
          <span className="text-border">|</span>
          <Kbd>{"⇧"}</Kbd>
          <span>+</span>
          <KbdGroup>
            <Kbd>{"↑"}</Kbd>
            <Kbd>{"↓"}</Kbd>
            <Kbd>{"←"}</Kbd>
            <Kbd>{"→"}</Kbd>
          </KbdGroup>
          <span>navigate nodes</span>
        </div>
      </Surface>
    </Command>
  );
}

export { CmdInput };
