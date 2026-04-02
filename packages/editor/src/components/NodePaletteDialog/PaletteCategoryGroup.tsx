"use client";

import { useCallback } from "react";
import { AccordionItem, AccordionTrigger, AccordionContent } from "@bnto/ui";
import type { PaletteGroup } from "../../hooks/useNodePalette";
import { CATEGORY_VARIANT } from "../../adapters/categoryVariant";
import { PaletteItem } from "./PaletteItem";

/**
 * PaletteCategoryGroup — collapsible section for one node category.
 *
 * Uses Accordion for expand/collapse. Renders PaletteItem for each
 * node type in the group.
 */

interface PaletteCategoryGroupProps {
  group: PaletteGroup;
  isFull: boolean;
  onAdd: (type: string) => void;
}

function PaletteCategoryGroup({ group, isFull, onAdd }: PaletteCategoryGroupProps) {
  const variant = CATEGORY_VARIANT[group.category.name] ?? "muted";
  const handleAdd = useCallback((type: string) => onAdd(type), [onAdd]);

  return (
    <AccordionItem value={group.category.name} className="border-b-0">
      <AccordionTrigger className="py-2 text-sm font-medium text-muted-foreground">
        {group.category.label}
      </AccordionTrigger>
      <AccordionContent className="pb-1 pt-0">
        <div className="flex flex-col gap-0.5">
          {group.items.map((item) => (
            <PaletteItem
              key={item.type}
              type={item.type}
              label={item.label}
              description={item.description}
              icon={item.icon}
              variant={variant}
              platforms={item.platforms}
              disabled={isFull || !item.platforms.includes("browser")}
              onAdd={handleAdd}
              testId={`palette-item-${item.type}`}
            />
          ))}
        </div>
      </AccordionContent>
    </AccordionItem>
  );
}

export { PaletteCategoryGroup };
