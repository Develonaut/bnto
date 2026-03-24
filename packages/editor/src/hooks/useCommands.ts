/**
 * useCommands — reactive hook for the CmdEditor command palette.
 *
 * Returns three levels of commands for the Notion-style drill-down:
 * 1. Root — global actions + "Add Node" entry
 * 2. Categories — one per palette group (triggered by "Add Node")
 * 3. Processors — node types within a category
 *
 * Navigation state (which level is active) lives in CmdInput, not here.
 * This hook just resolves the data for each level reactively.
 */

"use client";

import { useMemo, useCallback } from "react";
import { useStore } from "zustand";
import { useEditor, useEditorStoreApi } from "../context";
import { useNodePalette } from "./useNodePalette";
import { resolveCommands, groupCommands } from "../commands/resolveCommands";
import { buildCategoryCommands, buildProcessorCommands } from "../commands/addNodeCommands";
import type { CommandGroup, CommandEditorApi } from "../commands/types";
import type { PaletteGroup } from "./useNodePalette";

interface UseCommandsResult {
  /** Resolves the root-level command groups (Actions). */
  rootGroups: (onAddNode: () => void) => CommandGroup[];
  /** Resolves the category-level command groups (Choose Category). */
  categoryGroups: (onSelectCategory: (name: string) => void) => CommandGroup[];
  /** Resolves the processor-level command groups for a specific category. */
  processorGroups: (categoryName: string) => CommandGroup[];
}

function useCommands(): UseCommandsResult {
  const editor = useEditor();
  const storeApi = useEditorStoreApi();
  const { groups: paletteGroups } = useNodePalette(true);

  const selectedNodeId = useStore(storeApi, (s) => s.selectedNodeId);
  const configs = useStore(storeApi, (s) => s.configs);
  const executionPhase = useStore(storeApi, (s) => s.executionPhase);
  const executionInputFiles = useStore(storeApi, (s) => s.executionInputFiles);

  const state = useMemo(
    () => ({ selectedNodeId, configs, executionPhase, executionInputFiles }),
    [selectedNodeId, configs, executionPhase, executionInputFiles],
  );

  const rootGroups = useCallback(
    (onAddNode: () => void) => groupCommands(resolveCommands(editor, state, onAddNode)),
    [editor, state],
  );

  const categoryGroups = useCallback(
    (onSelectCategory: (name: string) => void) =>
      groupCommands(buildCategoryCommands(paletteGroups, onSelectCategory)),
    [paletteGroups],
  );

  const processorGroups = useCallback(
    (categoryName: string) => {
      const group = paletteGroups.find((g) => g.category.name === categoryName);
      if (!group) return [];
      return groupCommands(buildProcessorCommands(editor as CommandEditorApi, group.items));
    },
    [editor, paletteGroups],
  );

  return { rootGroups, categoryGroups, processorGroups };
}

export { useCommands };
export type { UseCommandsResult };
