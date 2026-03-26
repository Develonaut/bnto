import type { ElementType } from "react";

import Link from "next/link";
import { Slot } from "@radix-ui/react-slot";

/** Determine which element/component to render based on props. */
export function resolveComponent(
  as: ElementType | undefined,
  asChild: boolean,
  href?: string,
  target?: string,
): ElementType {
  if (asChild) return Slot;
  if (as) return as;
  if (!href) return "button";
  if (href.startsWith("/") && !target) return Link;
  return "a";
}
