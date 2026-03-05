"use client";

import { useState } from "react";
import type { ComponentProps } from "react";

import { Popover } from "../../overlay/Popover";

export function MenuRootWrapper({
  open: controlledOpen,
  onOpenChange,
  ...props
}: ComponentProps<typeof Popover>) {
  const [internalOpen, setInternalOpen] = useState(false);
  const open = controlledOpen ?? internalOpen;
  const handleOpenChange = (next: boolean) => {
    setInternalOpen(next);
    onOpenChange?.(next);
  };
  return <Popover open={open} onOpenChange={handleOpenChange} {...props} />;
}
