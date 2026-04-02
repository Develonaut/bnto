"use client";

import { useCallback } from "react";
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
  type: string;
  label: string;
  description: string;
  icon: string;
  variant: CompartmentVariant;
  platforms: readonly string[];
  disabled: boolean;
  onAdd: (type: string) => void;
  testId: string;
}

function PaletteItem({
  type,
  label,
  description,
  icon,
  variant,
  platforms,
  disabled,
  onAdd,
  testId,
}: PaletteItemProps) {
  const Icon = ICON_COMPONENTS[icon];
  const handleAdd = useCallback(() => onAdd(type), [onAdd, type]);

  return (
    <button
      type="button"
      onClick={handleAdd}
      disabled={disabled}
      data-testid={testId}
      className="flex items-start gap-2.5 rounded-md px-2 py-2 text-left transition-colors hover:bg-muted disabled:pointer-events-none disabled:opacity-50"
    >
      <PaletteItemIcon icon={Icon} variant={variant} />
      <PaletteItemContent label={label} description={description} platforms={platforms} />
    </button>
  );
}

function PaletteItemIcon({
  icon: Icon,
  variant,
}: {
  icon: (typeof ICON_COMPONENTS)[string];
  variant: CompartmentVariant;
}) {
  if (!Icon) return null;
  return (
    <IconBadge variant={variant} size="sm" className="mt-0.5 size-7 shrink-0">
      <Icon className="size-3.5" />
    </IconBadge>
  );
}

function PaletteItemContent({
  label,
  description,
  platforms,
}: {
  label: string;
  description: string;
  platforms: readonly string[];
}) {
  return (
    <div className="min-w-0 flex-1">
      <span className="flex w-full items-center gap-2">
        <Text size="sm" weight="medium" className="truncate">
          {label}
        </Text>
        {!platforms.includes("browser") && (
          <Badge variant="outline" className="shrink-0 text-[10px]">
            Pro
          </Badge>
        )}
      </span>
      <Text size="xs" color="muted" className="line-clamp-2">
        {description}
      </Text>
    </div>
  );
}

export { PaletteItem };
