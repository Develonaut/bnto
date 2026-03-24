/**
 * globalCommands — builds the visible palette commands.
 *
 * Only surfaces actions that are meaningful in the palette:
 * - Run (only when actually runnable — files loaded, not mid-run)
 * - Export
 * - Delete Node (only when a deletable node is selected)
 *
 * Undo/redo/keyboard-obvious shortcuts are handled by
 * useCmdEditorShortcuts and don't clutter the palette.
 */

import type { Command, CommandEditorApi, CommandState } from "./types";
import { resolveDelete, canTriggerRun } from "../actions/shortcutActions";
import { downloadDefinition } from "../actions/downloadDefinition";

function buildGlobalCommands(editor: CommandEditorApi, state: CommandState): Command[] {
  const { selectedNodeId, configs, executionPhase, executionInputFiles } = state;

  const commands: Command[] = [];

  // --- Suggestions: contextual actions based on current state ---

  // Run — only when there are files and we're not already running
  if (canTriggerRun(executionPhase, executionInputFiles.length)) {
    commands.push({
      id: "run",
      label: "Run",
      icon: "Play",
      group: "Suggestions",
      keywords: ["run", "execute", "start", "process"],
      shortcut: "\u2318\u23ce",
      execute: () => editor.execution.runExecution(executionInputFiles),
    });
  }

  // Delete Node — only when a deletable (non-IO) node is selected
  const nodeType = selectedNodeId ? (configs[selectedNodeId]?.nodeType ?? null) : null;
  const deleteTarget = resolveDelete(selectedNodeId, nodeType);
  if (deleteTarget) {
    commands.push({
      id: "delete-node",
      label: "Delete Node",
      icon: "Trash2",
      group: "Suggestions",
      keywords: ["delete", "remove", "trash"],
      shortcut: "\u232b",
      execute: () => editor.nodes.removeNode(deleteTarget),
    });
  }

  // --- Actions: stable commands always available ---

  commands.push({
    id: "export",
    label: "Download",
    icon: "Download",
    group: "Actions",
    keywords: ["export", "download", "save", "json"],
    shortcut: "\u2318D",
    execute: () => downloadDefinition(editor.definition),
  });

  return commands;
}

export { buildGlobalCommands };
