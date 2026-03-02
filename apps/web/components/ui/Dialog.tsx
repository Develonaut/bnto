"use client";

import { forwardRef } from "react";
import type { ElementRef, ComponentPropsWithoutRef, ComponentProps } from "react";

import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { XIcon } from "@/components/ui/icons";
import { cn } from "@/lib/cn";

/* ── Root ───────────────────────────────────────────────────── */

function DialogRoot(props: ComponentProps<typeof DialogPrimitive.Dialog>) {
  return <DialogPrimitive.Dialog {...props} />;
}

/* ── Trigger ────────────────────────────────────────────────── */

const DialogTrigger = DialogPrimitive.Trigger;

/* ── Close ──────────────────────────────────────────────────── */

/**
 * Composable close button. When used with `asChild`, wraps the child
 * in Radix's close primitive. When used standalone (no `asChild`),
 * renders the default X icon button.
 */
const DialogClose = forwardRef<
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
    <DialogPrimitive.Content
      ref={ref}
      asChild
      {...props}
    >
      {/* Centering wrapper — fixed fullscreen, z-50 above overlay,
          pointer-events-none so clicks outside dismiss via overlay. */}
      <div className="fixed inset-0 z-50 flex items-center justify-center pointer-events-none">
        <Animate.ScaleIn from={0.6} easing="spring-bouncier">
          <Card
            elevation="lg"
            className={cn("pointer-events-auto relative w-full max-w-lg p-8", className)}
          >
            {children}
          </Card>
        </Animate.ScaleIn>
      </div>
    </DialogPrimitive.Content>
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
    className={cn("flex items-center justify-between gap-4", className)}
    {...props}
  />
));
DialogHeader.displayName = "Dialog.Header";

/* ── Body ──────────────────────────────────────────────────── */

const DialogBody = forwardRef<
  HTMLDivElement,
  ComponentPropsWithoutRef<"div">
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("py-4", className)}
    {...props}
  />
));
DialogBody.displayName = "Dialog.Body";

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
    className={cn("font-display text-xl font-black tracking-tight", className)}
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
    className={cn("text-muted-foreground leading-snug", className)}
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
  Body: DialogBody,
  Footer: DialogFooter,
  Title: DialogTitle,
  Description: DialogDescription,
});
