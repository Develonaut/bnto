"use client";

import { useCallback } from "react";
import { FileCogIcon, useDialog } from "@bnto/ui";
import { useEditor } from "../../context";
import { downloadDefinition } from "../../actions/downloadDefinition";
import { EditorMenuPanel } from "../EditorMenuPanel";
import { RecipeDialog } from "../RecipeDialog";
import { OpenRecipeDialog } from "../OpenRecipeDialog";
import { RecipeSettingsPanel } from "../ConfigPanel/RecipeSettingsPanel";
import { RecipeActionsSection } from "./RecipeActionsSection";
import type { RecipeActionsProps } from "./RecipeActionsSection";

/**
 * SettingsPanel — dedicated left-side panel for recipe-level controls.
 *
 * Shows recipe settings (name, iteration mode) and recipe actions
 * (rename, new, export, import). Opened via the toolbar settings button.
 */

interface DialogHandle {
  openDialog: () => void;
}

/** Recipe action callbacks for the panel footer. */
function useRecipeActions(
  renameDialog: DialogHandle,
  importDialog: DialogHandle,
): RecipeActionsProps {
  const editor = useEditor();
  const { validationErrors } = editor.definition.useDefinition();
  const { nodes } = editor.nodes.useNodes();
  const canExport = validationErrors.length === 0 && nodes.length > 0;

  const handleNew = useCallback(() => editor.definition.createBlank(), [editor]);
  const handleExport = useCallback(() => downloadDefinition(editor.definition), [editor]);

  return {
    onRename: renameDialog.openDialog,
    onImport: importDialog.openDialog,
    onNew: handleNew,
    onExport: handleExport,
    canExport,
  };
}

function SettingsPanelRoot() {
  const renameDialog = useDialog();
  const importDialog = useDialog();
  const actions = useRecipeActions(renameDialog, importDialog);

  return (
    <>
      <EditorMenuPanel
        panelId="settings"
        side="right"
        width="w-72"
        boundaryPadding={16}
        label="Settings"
        icon={<FileCogIcon className="size-4" />}
      >
        <RecipeSettingsPanel />
        <RecipeActionsSection {...actions} />
      </EditorMenuPanel>
      <RecipeDialog open={renameDialog.open} onOpenChange={renameDialog.onOpenChange} />
      <OpenRecipeDialog open={importDialog.open} onOpenChange={importDialog.onOpenChange} />
    </>
  );
}

export { SettingsPanelRoot };
