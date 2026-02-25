"use client";

import * as React from "react";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/Card";
import {
  Popover,
  PopoverAnchor,
  PopoverContentUnstyled,
  PopoverPortal,
  PopoverTrigger,
} from "@/components/ui/Popover";
import { cn } from "@/lib/utils";

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
 * Renders as our Button component. When the menu is open, the
 * trigger shows hovered={true} (engaged, sunk toward ground). */

const MenuTrigger = React.forwardRef<
  HTMLButtonElement,
  React.ComponentProps<typeof Button>
>(({ children, ...props }, ref) => {
  const open = React.useContext(MenuContext);
  return (
    <PopoverAnchor asChild>
      <div className="inline-flex">
        <PopoverTrigger asChild>
          <Button ref={ref} hovered={open} {...props}>
            {children}
          </Button>
        </PopoverTrigger>
      </div>
    </PopoverAnchor>
  );
});
MenuTrigger.displayName = "MenuTrigger";

/* ── Content ─────────────────────────────────────────────────────
 * Renders as our Card component inside a Radix Popover.Content.
 * Handles positioning, collision detection, and enter/exit
 * animations via Radix. Card provides bg-card surface + depth. */

type MenuOffset = "sm" | "md" | "lg" | "xl";

const OFFSET_PX: Record<MenuOffset, number> = {
  sm: 8,
  md: 16,
  lg: 24,
  xl: 32,
};

const MenuContent = React.forwardRef<
  React.ElementRef<typeof PopoverContentUnstyled>,
  Omit<React.ComponentPropsWithoutRef<typeof PopoverContentUnstyled>, "sideOffset"> & {
    depth?: "none" | "sm" | "md" | "lg";
    offset?: MenuOffset;
    boundary?: Element | null;
    boundaryPadding?: number;
  }
>(
  (
    { className, children, offset = "md", depth = "lg", boundary, boundaryPadding = 16, ...props },
    ref,
  ) => (
    <PopoverPortal>
      <PopoverContentUnstyled
        ref={ref}
        sideOffset={OFFSET_PX[offset]}
        collisionBoundary={boundary ?? undefined}
        collisionPadding={boundaryPadding}
        className="z-50 outline-hidden"
        {...props}
      >
        <Animate.ScaleIn from={0.6} origin="top" easing="spring-bouncier">
          <Card className={className} depth={depth}>
            {children}
          </Card>
        </Animate.ScaleIn>
      </PopoverContentUnstyled>
    </PopoverPortal>
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

export const Menu = Object.assign(MenuRootWrapper, {
  Root: MenuRootWrapper,
  Trigger: MenuTrigger,
  Content: MenuContent,
});
