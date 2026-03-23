"use client";

import { PlusIcon, Text } from "@bnto/ui";

/**
 * NodeListPlaceholder — dashed-border empty state shown when
 * no processing nodes exist on the canvas.
 */
function NodeListPlaceholder() {
  return (
    <div className="flex items-center gap-2 rounded-md px-3 py-3 outline outline-2 outline-dashed outline-border -outline-offset-2">
      <PlusIcon className="size-4 shrink-0 text-muted-foreground" />
      <Text size="xs" color="muted">
        Add a node from the Palette
      </Text>
    </div>
  );
}

export { NodeListPlaceholder };
