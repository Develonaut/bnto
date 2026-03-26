"use client";

import type { ReactNode, ComponentProps } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogClose,
  DialogDescription,
} from "./Dialog";

type DialogSize = ComponentProps<typeof DialogContent>["size"];

interface DialogShellProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  /** Screen-reader-only description for a11y. */
  description: string;
  size?: DialogSize;
  children: ReactNode;
  /** Extra props on DialogContent (className, data-testid, style, etc). */
  contentProps?: Omit<ComponentProps<typeof DialogContent>, "size" | "children"> &
    Record<`data-${string}`, string>;
  /** Extra className on DialogHeader. */
  headerClassName?: string;
}

/**
 * DialogShell — convenience wrapper for the repeated Dialog chrome.
 *
 * Renders Dialog > DialogContent > DialogHeader (Title + Close) +
 * sr-only Description. Children go in the body area. For dialogs
 * that need full composition control (custom header, no close button),
 * use the raw Dialog primitives instead.
 */
function DialogShell({
  open,
  onOpenChange,
  title,
  description,
  size = "sm",
  children,
  contentProps,
  headerClassName,
}: DialogShellProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent size={size} {...contentProps}>
        <DialogHeader className={headerClassName}>
          <DialogTitle>{title}</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogDescription className="sr-only">{description}</DialogDescription>
        {children}
      </DialogContent>
    </Dialog>
  );
}

export { DialogShell };
export type { DialogShellProps };
