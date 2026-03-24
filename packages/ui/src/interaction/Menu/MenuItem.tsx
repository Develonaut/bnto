"use client";

import type { ComponentProps, ElementType } from "react";

import { Slot } from "@radix-ui/react-slot";

import { ListItem } from "../../layout/List";
import { PopoverClose } from "../../overlay/Popover";

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
    <PopoverClose asChild>
      <ListItem selectable asChild>
        <Comp ref={ref} type={asChild ? undefined : "button"} className={className} {...props} />
      </ListItem>
    </PopoverClose>
  );
}
