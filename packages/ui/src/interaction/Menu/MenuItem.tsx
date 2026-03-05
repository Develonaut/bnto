"use client";

import type { ComponentProps, ElementType } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../../utils/cn";
import { Popover } from "../../overlay/Popover";

const ITEM_BASE =
  "flex w-full items-center gap-2 rounded-lg px-4 py-2.5 text-sm transition-colors select-none outline-hidden [&_svg:not([class*='size-'])]:size-4 hover:bg-muted focus-visible:bg-muted disabled:pointer-events-none disabled:opacity-50";

/**
 * A styled interactive container for a single menu row.
 * Compose children freely -- icons, labels, descriptions.
 *
 * Wraps itself in Menu.Close so clicking any item closes the
 * menu automatically.
 *
 * Renders a <button> by default. Use `asChild` to merge onto a
 * child element (e.g. Next.js Link).
 */
export function MenuItem({
  asChild,
  className,
  ref,
  ...props
}: ComponentProps<"button"> & { asChild?: boolean }) {
  const Comp: ElementType = asChild ? Slot : "button";
  return (
    <Popover.Close asChild>
      <Comp
        ref={ref}
        type={asChild ? undefined : "button"}
        className={cn(ITEM_BASE, className)}
        {...props}
      />
    </Popover.Close>
  );
}
