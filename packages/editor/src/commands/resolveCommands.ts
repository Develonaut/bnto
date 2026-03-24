/**
 * resolveCommands — single entry point for top-level CmdEditor commands.
 *
 * Returns the root command list: global actions + "Add Node" drill-down
 * entry. Category and processor sub-commands are resolved separately
 * by the CmdInput navigation state (see addNodeCommands.ts).
 */

import type { Command, CommandGroup, CommandEditorApi, CommandState } from "./types";
import { buildGlobalCommands } from "./globalCommands";
import { buildAddNodeEntry } from "./addNodeCommands";

function resolveCommands(
  editor: CommandEditorApi,
  state: CommandState,
  onAddNodeDrillDown: () => void,
): Command[] {
  const addEntry = buildAddNodeEntry(onAddNodeDrillDown);
  const globals = buildGlobalCommands(editor, state);
  // Add Node is always the first suggestion
  return [addEntry, ...globals];
}

/** Groups a flat command list by the `group` field, preserving insertion order. */
function groupCommands(commands: Command[]): CommandGroup[] {
  const map = new Map<string, Command[]>();
  for (const cmd of commands) {
    const existing = map.get(cmd.group) ?? [];
    existing.push(cmd);
    map.set(cmd.group, existing);
  }
  return Array.from(map, ([group, commands]) => ({ group, commands }));
}

export { resolveCommands, groupCommands };
