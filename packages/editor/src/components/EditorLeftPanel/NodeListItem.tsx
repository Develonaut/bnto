"use client";

import { useCallback } from "react";
import { Text, Button, TrashIcon, IconBadge } from "@bnto/ui";
import { ICON_COMPONENTS } from "../../adapters/nodeIcons";
import type { CompartmentVariant } from "../../adapters/types";

/**
 * NodeListItem — single row in the Nodes list tab.
 *
 * Displays icon (variant-colored) + label. Click selects on canvas.
 * Hover reveals a delete button (hidden for I/O nodes).
 */

interface NodeListItemProps {
  nodeId: string;
  label: string;
  icon?: string;
  variant: CompartmentVariant;
  selected: boolean;
  isIoNode: boolean;
  onSelect: (id: string) => void;
  onRemove: (id: string) => void;
}

function NodeListItem({
  nodeId,
  label,
  icon,
  variant,
  selected,
  isIoNode,
  onSelect,
  onRemove,
}: NodeListItemProps) {
  const Icon = icon ? ICON_COMPONENTS[icon] : undefined;

  const handleClick = useCallback(() => onSelect(nodeId), [nodeId, onSelect]);

  const handleRemove = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onRemove(nodeId);
    },
    [nodeId, onRemove],
  );

  return (
    <button
      type="button"
      onClick={handleClick}
      data-active={selected}
      className="group flex w-full items-center gap-2.5 rounded-md px-2 py-1.5 text-left transition-colors hover:bg-muted data-[active=true]:bg-muted"
      data-testid={`node-list-item-${nodeId}`}
    >
      {Icon && (
        <IconBadge variant={variant} size="sm" className="size-7 shrink-0">
          <Icon className="size-3.5" />
        </IconBadge>
      )}
      <Text size="sm" className="min-w-0 flex-1 truncate">
        {label}
      </Text>
      {!isIoNode && (
        <Button
          icon={<TrashIcon />}
          variant="ghost"
          size="icon"
          className="size-6 shrink-0 opacity-0 group-hover:opacity-100 group-focus-within:opacity-100"
          onClick={handleRemove}
          aria-label={`Remove ${label}`}
          data-testid={`node-list-remove-${nodeId}`}
        />
      )}
    </button>
  );
}

export { NodeListItem };
