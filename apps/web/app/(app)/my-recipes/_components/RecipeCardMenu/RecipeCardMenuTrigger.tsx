"use client";

import {
  EllipsisVerticalIcon,
  Menu,
  MenuContent,
  MenuItem,
  MenuTrigger,
  PenLineIcon,
  TrashIcon,
} from "@bnto/ui";

interface RecipeCardMenuTriggerProps {
  recipeName: string;
  onRenameOpen: () => void;
  onDeleteOpen: () => void;
}

/** Ellipsis menu trigger with Rename + Delete items. */
export function RecipeCardMenuTrigger({
  recipeName,
  onRenameOpen,
  onDeleteOpen,
}: RecipeCardMenuTriggerProps) {
  return (
    <Menu>
      <MenuTrigger
        icon={<EllipsisVerticalIcon />}
        variant="primary"
        dormant
        aria-label={`Actions for ${recipeName}`}
        data-testid="recipe-menu"
      />
      <MenuContent align="end">
        <MenuItem onClick={onRenameOpen}>
          <PenLineIcon />
          Rename
        </MenuItem>
        <MenuItem onClick={onDeleteOpen} className="text-destructive">
          <TrashIcon />
          Delete
        </MenuItem>
      </MenuContent>
    </Menu>
  );
}
