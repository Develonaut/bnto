"use client";

import * as React from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "@/lib/cn";
import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Popover } from "@/components/ui/Popover";

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

/* ── Context ──────────────────────────────────────────────────────
 * Shares the open state from MenuRoot so the trigger can force
 * itself into the hover (engaged) force state. */

const MenuContext = React.createContext(false);

/* ── Trigger ─────────────────────────────────────────────────────
 * Renders as our Button component with toggle behavior. When the
 * menu is open, the trigger stays depressed at hover depth. */

const MenuTrigger = React.forwardRef<
  HTMLButtonElement,
  React.ComponentProps<typeof Button>
>(({ children, ...props }, ref) => {
  const open = React.useContext(MenuContext);
  return (
    <Popover.Trigger asChild>
      <Button ref={ref} toggle pressed={open} {...props}>
        {children}
      </Button>
    </Popover.Trigger>
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

const MenuContent = React.forwardRef<
  React.ElementRef<typeof Popover.ContentUnstyled>,
  Omit<
    React.ComponentPropsWithoutRef<typeof Popover.ContentUnstyled>,
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
        sideOffset={OFFSET_PX[offset]}
        collisionBoundary={boundary ?? undefined}
        collisionPadding={boundaryPadding}
        className="z-50 outline-hidden"
        {...props}
      >
        <Animate.ScaleIn from={0.6} origin="top" easing="spring-bouncier">
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
}: React.ComponentProps<typeof Popover>) {
  const [internalOpen, setInternalOpen] = React.useState(false);
  const open = controlledOpen ?? internalOpen;
  const handleOpenChange = (next: boolean) => {
    setInternalOpen(next);
    onOpenChange?.(next);
  };
  return (
    <MenuContext.Provider value={open}>
      <Popover open={open} onOpenChange={handleOpenChange} {...props} />
    </MenuContext.Provider>
  );
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
}: React.ComponentProps<"button"> & { asChild?: boolean }) {
  const Comp: React.ElementType = asChild ? Slot : "button";
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

function MenuLabel({
  className,
  ...props
}: React.ComponentProps<"div">) {
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

function MenuSeparator({
  className,
  ...props
}: React.ComponentProps<"div">) {
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
