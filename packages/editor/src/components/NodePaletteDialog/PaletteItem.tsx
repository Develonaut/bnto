"use client";

import { Badge, Text, IconBadge } from "@bnto/ui";
import { ICON_COMPONENTS } from "../../adapters/nodeIcons";
import type { CompartmentVariant } from "../../adapters/types";

/**
 * PaletteItem — single clickable row in the palette list.
 *
 * Shows icon + label + description. Server-only nodes get a Pro badge.
 * Click calls onAdd to add the node to the canvas.
 */

interface PaletteItemProps {
  label: string;
  description: string;
  icon: string;
  variant: CompartmentVariant;
  browserCapable: boolean;
  disabled: boolean;
  onAdd: () => void;
  testId: string;
}

function PaletteItem({
  label,
  description,
  icon,
  variant,
  browserCapable,
  disabled,
  onAdd,
  testId,
}: PaletteItemProps) {
  const isServerOnly = !browserCapable;
  const Icon = ICON_COMPONENTS[icon];

  return (
    <button
      type="button"
      onClick={onAdd}
      disabled={disabled}
      data-testid={testId}
      className="flex items-start gap-2.5 rounded-md px-2 py-2 text-left transition-colors hover:bg-muted disabled:pointer-events-none disabled:opacity-50"
    >
      {Icon && (
        <IconBadge variant={variant} size="sm" className="mt-0.5 size-7 shrink-0">
          <Icon className="size-3.5" />
        </IconBadge>
      )}
      <div className="min-w-0 flex-1">
        <span className="flex w-full items-center gap-2">
          <Text size="sm" weight="medium" className="truncate">
            {label}
          </Text>
          {isServerOnly && (
            <Badge variant="outline" className="shrink-0 text-[10px]">
              Pro
            </Badge>
          )}
        </span>
        <Text size="xs" color="muted" className="line-clamp-2">
          {description}
        </Text>
      </div>
    </button>
  );
}

export { PaletteItem };
