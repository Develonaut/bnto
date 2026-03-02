"use client";

import { forwardRef } from "react";
import type { ElementRef, ComponentPropsWithoutRef, ComponentProps } from "react";

import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { XIcon } from "@/components/ui/icons";
import { cn } from "@/lib/cn";

/* ── Root ───────────────────────────────────────────────────── */

function DialogRoot(props: ComponentProps<typeof DialogPrimitive.Dialog>) {
  return <DialogPrimitive.Dialog {...props} />;
}

/* ── Trigger ────────────────────────────────────────────────── */

const DialogTrigger = DialogPrimitive.Trigger;

/* ── Close ──────────────────────────────────────────────────── */

const DialogClose = DialogPrimitive.Close;

/* ── Portal ─────────────────────────────────────────────────── */

const DialogPortal = DialogPrimitive.Portal;

/* ── Overlay ────────────────────────────────────────────────── */

const DialogOverlay = forwardRef<
  ElementRef<typeof DialogPrimitive.Overlay>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Overlay
    ref={ref}
    className={cn(
      "fixed inset-0 z-50 bg-background/40 backdrop-blur-sm",
      className,
    )}
    {...props}
  />
));
DialogOverlay.displayName = "Dialog.Overlay";

/* ── Content ────────────────────────────────────────────────── */

const DialogContent = forwardRef<
  ElementRef<typeof DialogPrimitive.Content>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Content>
>(({ className, children, ...props }, ref) => (
  <DialogPortal>
    <DialogOverlay />
    {/* Positioning wrapper — establishes z-50 stacking context so the
        Animate.ScaleIn wrapper (which creates its own stacking context
        from the CSS animation) doesn't fall behind the overlay. */}
    <div className="fixed inset-0 z-50 flex items-center justify-center pointer-events-none">
      <Animate.ScaleIn from={0.92} easing="spring-bouncy">
        <DialogPrimitive.Content
          ref={ref}
          className={cn(
            "pointer-events-auto w-full max-w-lg",
            "rounded-xl border border-border bg-card p-6 shadow-xl",
            className,
          )}
          {...props}
        >
          {children}
          <div className="absolute top-3 right-3 z-10">
            <DialogPrimitive.Close asChild>
              <Button variant="ghost" size="icon" elevation="md" className="size-8">
                <XIcon className="size-4" />
                <span className="sr-only">Close</span>
              </Button>
            </DialogPrimitive.Close>
          </div>
        </DialogPrimitive.Content>
      </Animate.ScaleIn>
    </div>
  </DialogPortal>
));
DialogContent.displayName = "Dialog.Content";

/* ── Header ─────────────────────────────────────────────────── */

const DialogHeader = forwardRef<
  HTMLDivElement,
  ComponentPropsWithoutRef<"div">
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex flex-col gap-1.5 pr-10 text-center sm:text-left", className)}
    {...props}
  />
));
DialogHeader.displayName = "Dialog.Header";

/* ── Footer ─────────────────────────────────────────────────── */

const DialogFooter = forwardRef<
  HTMLDivElement,
  ComponentPropsWithoutRef<"div">
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex flex-col-reverse gap-2 pt-4 sm:flex-row sm:justify-end", className)}
    {...props}
  />
));
DialogFooter.displayName = "Dialog.Footer";

/* ── Title ──────────────────────────────────────────────────── */

const DialogTitle = forwardRef<
  ElementRef<typeof DialogPrimitive.Title>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Title>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Title
    ref={ref}
    className={cn("font-display text-lg font-semibold leading-none tracking-tight", className)}
    {...props}
  />
));
DialogTitle.displayName = "Dialog.Title";

/* ── Description ────────────────────────────────────────────── */

const DialogDescription = forwardRef<
  ElementRef<typeof DialogPrimitive.Description>,
  ComponentPropsWithoutRef<typeof DialogPrimitive.Description>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Description
    ref={ref}
    className={cn("text-sm text-muted-foreground", className)}
    {...props}
  />
));
DialogDescription.displayName = "Dialog.Description";

/* ── Namespace ──────────────────────────────────────────────── */

export const Dialog = Object.assign(DialogRoot, {
  Root: DialogRoot,
  Trigger: DialogTrigger,
  Close: DialogClose,
  Portal: DialogPortal,
  Overlay: DialogOverlay,
  Content: DialogContent,
  Header: DialogHeader,
  Footer: DialogFooter,
  Title: DialogTitle,
  Description: DialogDescription,
});
