"use client";

import { useCallback } from "react";
import {
  Row,
  Text,
  Menu,
  MenuTrigger,
  MenuContent,
  MenuItem,
  MenuSeparator,
  PenLineIcon,
  PlusIcon,
  DownloadIcon,
  FileUpIcon,
  EllipsisVerticalIcon,
  useDialog,
} from "@bnto/ui";
import { useEditor } from "../../context";
import { downloadDefinition } from "../../actions/downloadDefinition";
import { RecipeDialog } from "../RecipeDialog";
import { OpenRecipeDialog } from "../OpenRecipeDialog";

/**
 * RecipeFileMenu — recipe name display + file operations menu.
 *
 * Shows the current recipe name with an ellipsis menu for
 * rename, new, export, and import actions.
 */
function RecipeFileMenu() {
  const editor = useEditor();
  const { recipeMetadata, validationErrors } = editor.definition.useDefinition();
  const { nodes } = editor.nodes.useNodes();
  const renameDialog = useDialog();
  const importDialog = useDialog();

  const hasNodes = nodes.length > 0;
  const canDownload = validationErrors.length === 0 && hasNodes;

  const handleNew = useCallback(() => {
    editor.definition.createBlank();
  }, [editor]);

  const handleExport = useCallback(() => {
    downloadDefinition(editor.definition);
  }, [editor]);

  return (
    <>
      <Row className="items-center gap-1">
        <Text size="sm" weight="medium" className="min-w-0 flex-1 truncate">
          {recipeMetadata.name}
        </Text>
        <Menu>
          <MenuTrigger
            icon={<EllipsisVerticalIcon />}
            variant="ghost"
            elevation="sm"
            aria-label="File menu"
            data-testid="panel-file-menu"
          />
          <MenuContent side="bottom" className="w-48 p-1">
            <MenuItem onClick={renameDialog.openDialog} data-testid="panel-menu-rename">
              <PenLineIcon /> Rename
            </MenuItem>
            <MenuItem onClick={handleNew} data-testid="panel-menu-new">
              <PlusIcon /> New Recipe
            </MenuItem>
            <MenuSeparator />
            <MenuItem
              onClick={handleExport}
              disabled={!canDownload}
              data-testid="panel-menu-export"
            >
              <DownloadIcon /> Export
            </MenuItem>
            <MenuItem onClick={importDialog.openDialog} data-testid="panel-menu-import">
              <FileUpIcon /> Import
            </MenuItem>
          </MenuContent>
        </Menu>
      </Row>

      <RecipeDialog open={renameDialog.open} onOpenChange={renameDialog.onOpenChange} />
      <OpenRecipeDialog open={importDialog.open} onOpenChange={importDialog.onOpenChange} />
    </>
  );
}

export { RecipeFileMenu };
