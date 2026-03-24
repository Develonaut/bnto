/**
 * Command registry types for the CmdEditor command palette.
 *
 * Commands are plain objects — no React dependency. The palette
 * resolves them from editor state and renders them via cmdk.
 */

import type { NodeTypeName } from "@bnto/core";
import type { DefinitionClient, PanelClient } from "../editorTypes";
import type { ExecutionPhase } from "../store/types";
import type { NodeConfigs } from "../adapters/types";

/**
 * Minimal editor API that command builders need.
 *
 * Satisfied by both EditorInstance and ReactEditorInstance —
 * avoids coupling to internal fields like _storeApi.
 */
interface CommandEditorApi {
  nodes: {
    addNode(type: NodeTypeName): string | null;
    removeNode(id: string): boolean;
    selectNode(id: string | null): void;
  };
  execution: { runExecution(files: File[]): Promise<void> };
  definition: DefinitionClient;
  panels: Pick<PanelClient, "togglePanel">;
}

interface Command {
  id: string;
  label: string;
  icon?: string;
  group: string;
  keywords?: string[];
  shortcut?: string;
  disabled?: boolean;
  execute: () => void;
}

/** Commands grouped by their group label, preserving insertion order. */
interface CommandGroup {
  group: string;
  commands: Command[];
}

/** Minimal state slice needed by command builders. */
interface CommandState {
  selectedNodeId: string | null;
  configs: NodeConfigs;
  executionPhase: ExecutionPhase;
  executionInputFiles: File[];
}

export type { Command, CommandGroup, CommandEditorApi, CommandState };
