/**
 * addNodeCommands — builds the guided "Add Node" drill-down commands.
 *
 * Two levels:
 * 1. Category commands — one per category with node types (Image, File, etc.)
 * 2. Processor commands — one per node type within a category
 *
 * The CmdInput component manages the drill-down navigation state.
 * These builders just produce the command objects for each level.
 */

import type { PaletteGroup, PaletteItem } from "../hooks/useNodePalette";
import type { Command, CommandEditorApi } from "./types";

/** Top-level "Add Node" entry that triggers drill-down into categories. */
function buildAddNodeEntry(onDrillDown: () => void): Command {
  return {
    id: "add-node",
    label: "Add Node",
    icon: "Plus",
    group: "Suggestions",
    keywords: ["add", "new", "node", "insert", "create"],
    execute: onDrillDown,
  };
}

/** Category-level commands — one per non-empty palette group. */
function buildCategoryCommands(
  groups: PaletteGroup[],
  onSelectCategory: (categoryName: string) => void,
): Command[] {
  return groups.map((group) => ({
    id: `cat-${group.category.name}`,
    label: group.category.label,
    icon: group.items[0]?.icon,
    group: "Choose Category",
    keywords: [
      group.category.name,
      group.category.label.toLowerCase(),
      group.category.description.toLowerCase(),
    ],
    execute: () => onSelectCategory(group.category.name),
  }));
}

/** Processor-level commands — one per node type within a category. */
function buildProcessorCommands(editor: CommandEditorApi, items: PaletteItem[]): Command[] {
  return items.map((item) => ({
    id: `add-${item.type}`,
    label: item.label,
    icon: item.icon,
    group: "Choose Processor",
    keywords: [item.label.toLowerCase(), item.description.toLowerCase()],
    execute: () => editor.nodes.addNode(item.type),
  }));
}

export { buildAddNodeEntry, buildCategoryCommands, buildProcessorCommands };
