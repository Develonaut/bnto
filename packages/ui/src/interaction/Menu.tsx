"use client";

import { forwardRef, useState } from "react";
import type {
  ComponentProps,
  ComponentPropsWithoutRef,
  ElementRef,
  ElementType,
} from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../utils/cn";
import { Animate } from "../animation/Animate";
import { Button } from "./Button";
import { Card } from "../surface/Card";
import { Popover } from "../overlay/Popover";

/* ── Menu ────────────────────────────────────────────────────────
 * A composed menu component: Button trigger + Card dropdown.
 * Built on Radix Popover for positioning and open/close behavior.
 *
 * Usage:
 *   <Menu>
 *     <Menu.Trigger variant="outline">Recipes</Menu.Trigger>
 *     <Menu.Content>
 *       ...anything...
 *     </Menu.Content>
 *   </Menu>
 * ──────────────────────────────────────────────────────────────── */

/* ── Trigger ─────────────────────────────────────────────────────
 * Renders as our Button component inside a Popover.Anchor wrapper.
 *
 * The Button uses .pressable which animates via CSS transform.
 * Without the Anchor, Radix uses the Trigger itself as the
 * Floating UI reference — so getBoundingClientRect() shifts as
 * the Button's spring animation plays, dragging the dropdown
 * with it.
 *
 * Popover.Anchor renders a static <div> that wraps the Trigger.
 * Radix detects the custom anchor and tells Floating UI to read
 * position from the Anchor instead of the Trigger. Since the
 * Anchor has no transform, its bounding rect never changes —
 * the dropdown stays pinned while the Button animates freely. */

const MenuTrigger = forwardRef<
  HTMLButtonElement,
  ComponentProps<typeof Button>
>(({ children, ...props }, ref) => {
  return (
    <Popover.Anchor className="inline-flex">
      <Popover.Trigger asChild>
        <Button ref={ref} {...props}>
          {children}
        </Button>
      </Popover.Trigger>
    </Popover.Anchor>
  );
});
MenuTrigger.displayName = "MenuTrigger";

/* ── Content ─────────────────────────────────────────────────────
 * Renders as our Card component inside a Radix Popover.Content.
 * Handles positioning, collision detection, and enter/exit
 * animations via Radix. Card provides bg-card surface + elevation. */

type MenuOffset = "sm" | "md" | "lg" | "xl";

const OFFSET_PX: Record<MenuOffset, number> = {
  sm: 8,
  md: 16,
  lg: 24,
  xl: 32,
};

/** Map Radix side → ScaleIn origin so the menu scales from the edge nearest the trigger. */
const SIDE_ORIGIN = {
  top: "bottom",
  bottom: "top",
  left: "right",
  right: "left",
} as const;

const MenuContent = forwardRef<
  ElementRef<typeof Popover.ContentUnstyled>,
  Omit<
    ComponentPropsWithoutRef<typeof Popover.ContentUnstyled>,
    "sideOffset"
  > & {
    elevation?: "none" | "sm" | "md" | "lg";
    offset?: MenuOffset;
    boundary?: Element | null;
    boundaryPadding?: number;
  }
>(
  (
    {
      className,
      children,
      side = "bottom",
      offset = "md",
      elevation = "lg",
      boundary,
      boundaryPadding = 16,
      ...props
    },
    ref,
  ) => (
    <Popover.Portal>
      <Popover.ContentUnstyled
        ref={ref}
        side={side}
        sideOffset={OFFSET_PX[offset]}
        collisionBoundary={boundary ?? undefined}
        collisionPadding={boundaryPadding}
        className="z-50 outline-hidden"
        {...props}
      >
        <Animate.ScaleIn
          from={0.6}
          origin={SIDE_ORIGIN[side]}
          easing="spring-bouncier"
        >
          <Card className={className} elevation={elevation}>
            {children}
          </Card>
        </Animate.ScaleIn>
      </Popover.ContentUnstyled>
    </Popover.Portal>
  ),
);
MenuContent.displayName = "MenuContent";

function MenuRootWrapper({
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

/** Closes the menu when its child is clicked. Renders no wrapper element. */
const MenuClose = Popover.Close;

/* ── Item ───────────────────────────────────────────────────────
 * A styled interactive container for a single menu row.
 * Compose children freely — icons, labels, descriptions.
 *
 * Wraps itself in Menu.Close so clicking any item closes the
 * menu automatically.
 *
 * Renders a <button> by default. Use `asChild` to merge onto a
 * child element (e.g. Next.js Link).
 *
 * Usage:
 *   // Action item — compose icon + label
 *   <Menu.Item onClick={handleSignOut}>
 *     <LogOutIcon />
 *     Sign out
 *   </Menu.Item>
 *
 *   // Rich item — compose as a link with any inner layout
 *   <Menu.Item asChild>
 *     <Link href="/compress-images">
 *       <span className="font-medium">Compress Images</span>
 *       <span className="text-xs text-muted-foreground">Shrink files</span>
 *     </Link>
 *   </Menu.Item>
 * ──────────────────────────────────────────────────────────────── */

const ITEM_BASE =
  "flex w-full items-center gap-2 rounded-lg px-4 py-2.5 text-sm transition-colors select-none outline-hidden [&_svg:not([class*='size-'])]:size-4 hover:bg-muted focus-visible:bg-muted disabled:pointer-events-none disabled:opacity-50";

function MenuItem({
  asChild,
  className,
  ref,
  ...props
}: ComponentProps<"button"> & { asChild?: boolean }) {
  const Comp: ElementType = asChild ? Slot : "button";
  return (
    <MenuClose asChild>
      <Comp
        ref={ref}
        type={asChild ? undefined : "button"}
        className={cn(ITEM_BASE, className)}
        {...props}
      />
    </MenuClose>
  );
}

/* ── Label ──────────────────────────────────────────────────────
 * Non-interactive category header inside a menu. Uppercase, muted,
 * small text. Used to group items into sections.
 *
 * Usage:
 *   <Menu.Label>Image</Menu.Label>
 * ──────────────────────────────────────────────────────────────── */

function MenuLabel({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "px-3 pt-3 pb-1 text-xs font-medium uppercase tracking-wider text-muted-foreground",
        className,
      )}
      {...props}
    />
  );
}

/* ── Separator ──────────────────────────────────────────────────
 * A thin horizontal line to separate groups of items.
 * ──────────────────────────────────────────────────────────────── */

function MenuSeparator({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      role="separator"
      className={cn("my-1 h-px bg-border", className)}
      {...props}
    />
  );
}

export const Menu = Object.assign(MenuRootWrapper, {
  Root: MenuRootWrapper,
  Trigger: MenuTrigger,
  Content: MenuContent,
  Close: MenuClose,
  Item: MenuItem,
  Label: MenuLabel,
  Separator: MenuSeparator,
});
