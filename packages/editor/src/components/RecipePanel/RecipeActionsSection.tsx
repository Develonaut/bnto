"use client";

import type { ReactNode } from "react";
import {
  Button,
  Divider,
  DownloadIcon,
  FileUpIcon,
  Heading,
  PenLineIcon,
  PlusIcon,
} from "@bnto/ui";
import { ShortcutHint } from "../ShortcutHint";

interface RecipeActionsProps {
  onRename: () => void;
  onImport: () => void;
  onNew: () => void;
  onExport: () => void;
  canExport: boolean;
}

/** Recipe file actions — rename, new, export, import. */
function RecipeActionsSection({
  onRename,
  onImport,
  onNew,
  onExport,
  canExport,
}: RecipeActionsProps) {
  return (
    <>
      <Divider />
      <div className="shrink-0 px-3 pt-3 pb-2">
        <Heading level={4} size="xs">
          Actions
        </Heading>
      </div>
      <div className="flex flex-col gap-1 px-2 pb-3">
        <ActionButton
          icon={<PenLineIcon />}
          label="Rename"
          onClick={onRename}
          testId="panel-rename"
        />
        <ActionButton icon={<PlusIcon />} label="New Recipe" onClick={onNew} testId="panel-new" />
        <ActionButton
          icon={<DownloadIcon />}
          label="Export"
          onClick={onExport}
          disabled={!canExport}
          testId="panel-export"
          shortcutId="export"
        />
        <ActionButton
          icon={<FileUpIcon />}
          label="Import"
          onClick={onImport}
          testId="panel-import"
        />
      </div>
    </>
  );
}

function ActionButton({
  icon,
  label,
  onClick,
  disabled,
  testId,
  shortcutId,
}: {
  icon: ReactNode;
  label: string;
  onClick: () => void;
  disabled?: boolean;
  testId: string;
  shortcutId?: string;
}) {
  return (
    <Button
      variant="ghost"
      size="sm"
      className="justify-start gap-2"
      onClick={onClick}
      disabled={disabled}
      data-testid={testId}
    >
      {icon}
      {label}
      {shortcutId && <ShortcutHint shortcutId={shortcutId} />}
    </Button>
  );
}

export { RecipeActionsSection };
export type { RecipeActionsProps };
