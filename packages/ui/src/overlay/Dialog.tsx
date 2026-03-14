"use client";

import { forwardRef } from "react";
import type { ElementRef, ComponentPropsWithoutRef, ComponentProps } from "react";

import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Button } from "../interaction/Button";
import { XIcon } from "../icons";
import { cn } from "../utils/cn";
import { Popup } from "./Popup";

/* ── Root ───────────────────────────────────────────────────── */

export function Dialog(props: ComponentProps<typeof DialogPrimitive.Dialog>) {
  return <DialogPrimitive.Dialog {...props} />;
}

/* ── Trigger ────────────────────────────────────────────────── */

export const DialogTrigger = DialogPrimitive.Trigger;

/* ── Close ──────────────────────────────────────────────────── */

/**
 * Composable close button. When used with `asChild`, wraps the child
 * in Radix's close primitive. When used standalone (no `asChild`),
 * renders the default X icon button.
 */
export const DialogClose = forwardRef<
  ElementRef<typeof DialogPrimitive.Close>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Close>
>(({ children, asChild, className, ...props }, ref) => {
  if (asChild || children) {
    return (
      <DialogPrimitive.Close ref={ref} asChild={asChild} className={className} {...props}>
        {children}
      </DialogPrimitive.Close>
    );
  }

  return (
    <DialogPrimitive.Close ref={ref} asChild {...props}>
      <Button variant="ghost" size="icon" elevation="md" className={cn("size-8", className)}>
        <XIcon className="size-4" />
        <span className="sr-only">Close</span>
      </Button>
    </DialogPrimitive.Close>
  );
});
DialogClose.displayName = "Dialog.Close";

/* ── Portal ─────────────────────────────────────────────────── */

export const DialogPortal = DialogPrimitive.Portal;

/* ── Overlay ────────────────────────────────────────────────── */

export const DialogOverlay = forwardRef<
  ElementRef<typeof DialogPrimitive.Overlay>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Overlay
    ref={ref}
    className={cn("fixed inset-0 z-modal bg-background/40 backdrop-blur-sm", className)}
    {...props}
  />
));
DialogOverlay.displayName = "Dialog.Overlay";

/* ── Content ────────────────────────────────────────────────── */

type DialogSize = "sm" | "md" | "lg" | "xl" | "2xl";

const DIALOG_MAX_WIDTH: Record<DialogSize, string> = {
  sm: "24rem",
  md: "32rem",
  lg: "42rem",
  xl: "56rem",
  "2xl": "72rem",
};

type DialogContentProps = ComponentPropsWithoutRef<typeof DialogPrimitive.Content> & {
  /** Width preset. Default "md". */
  size?: DialogSize;
};

export const DialogContent = forwardRef<ElementRef<typeof DialogPrimitive.Content>, DialogContentProps>(
  ({ className, children, size = "md", style, ...props }, ref) => (
    <DialogPortal>
      <DialogOverlay />
      <DialogPrimitive.Content ref={ref} asChild {...props}>
        {/* Centering wrapper — fixed fullscreen, z-modal above overlay,
            pointer-events-none so clicks outside dismiss via overlay. */}
        <div className="fixed inset-0 z-modal flex items-center justify-center pointer-events-none">
          <Popup
            elevation="lg"
            className={cn("pointer-events-auto relative p-8", className)}
            style={{ width: "100%", maxWidth: DIALOG_MAX_WIDTH[size], ...style }}
          >
            {children}
          </Popup>
        </div>
      </DialogPrimitive.Content>
    </DialogPortal>
  ),
);
DialogContent.displayName = "Dialog.Content";

/* ── Header ─────────────────────────────────────────────────── */

export const DialogHeader = forwardRef<HTMLDivElement, ComponentPropsWithoutRef<"div">>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn("flex items-center justify-between gap-4", className)}
      {...props}
    />
  ),
);
DialogHeader.displayName = "Dialog.Header";

/* ── Body ──────────────────────────────────────────────────── */

export const DialogBody = forwardRef<HTMLDivElement, ComponentPropsWithoutRef<"div">>(
  ({ className, ...props }, ref) => <div ref={ref} className={cn("py-4", className)} {...props} />,
);
DialogBody.displayName = "Dialog.Body";

/* ── Footer ─────────────────────────────────────────────────── */

export const DialogFooter = forwardRef<HTMLDivElement, ComponentPropsWithoutRef<"div">>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn("flex flex-col-reverse gap-2 pt-4 sm:flex-row sm:justify-end", className)}
      {...props}
    />
  ),
);
DialogFooter.displayName = "Dialog.Footer";

/* ── Title ──────────────────────────────────────────────────── */

export const DialogTitle = forwardRef<
  ElementRef<typeof DialogPrimitive.Title>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Title>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Title
    ref={ref}
    className={cn("font-display text-xl font-black tracking-tight", className)}
    {...props}
  />
));
DialogTitle.displayName = "Dialog.Title";

/* ── Description ────────────────────────────────────────────── */

export const DialogDescription = forwardRef<
  ElementRef<typeof DialogPrimitive.Description>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Description>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Description
    ref={ref}
    className={cn("text-muted-foreground leading-snug", className)}
    {...props}
  />
));
DialogDescription.displayName = "Dialog.Description";
