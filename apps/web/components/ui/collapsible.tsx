"use client";

import type { ComponentProps } from "react";

import * as CollapsiblePrimitive from "@radix-ui/react-collapsible";

const CollapsibleTrigger = CollapsiblePrimitive.CollapsibleTrigger;

const CollapsibleContent = CollapsiblePrimitive.CollapsibleContent;

function CollapsibleRoot(
  props: ComponentProps<typeof CollapsiblePrimitive.Root>,
) {
  return <CollapsiblePrimitive.Root {...props} />;
}

const Collapsible = Object.assign(CollapsibleRoot, {
  Root: CollapsibleRoot,
  Trigger: CollapsibleTrigger,
  Content: CollapsibleContent,
});

export { Collapsible, CollapsibleTrigger, CollapsibleContent };
