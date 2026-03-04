import { forwardRef } from "react";
import type { ComponentProps, HTMLAttributes } from "react";

import { cn } from "@/lib/cn";
import { createCn } from "./createCn";

import { Card } from "./Card";

/**
 * Toolbar — pill-shaped card surface with grouped controls.
 *
 * Shared foundation for the app navbar, editor floating toolbar, and
 * any future toolbar surfaces. Composes Card (Surface) with flex layout.
 *
 * Two sizes:
 *   default — navbar (px-6 py-3, gap-3.5)
 *   sm      — compact floating toolbar (px-3 py-1.5, gap-2)
 *
 * Usage:
 *   <Toolbar>
 *     <Toolbar.Group>
 *       <Button>A</Button>
 *       <Button>B</Button>
 *     </Toolbar.Group>
 *     <Toolbar.Divider />
 *     <Toolbar.Group>
 *       <Button>C</Button>
 *     </Toolbar.Group>
 *   </Toolbar>
 */

/* ── Variants ──────────────────────────────────────────────── */

const toolbarCn = createCn({
  base: "flex items-center rounded-full",
  variants: {
    size: {
      default: "gap-3.5 px-6 py-3",
      sm: "gap-2 px-3 py-1.5",
    },
  },
  defaultVariants: { size: "default" },
});

type ToolbarSize = "default" | "sm";

/* ── Root ──────────────────────────────────────────────────── */

type ToolbarRootProps = ComponentProps<typeof Card> & {
  /** Padding and gap scale. Default = navbar, sm = compact floating bar. */
  size?: ToolbarSize;
};

const ToolbarRoot = forwardRef<HTMLDivElement, ToolbarRootProps>(
  ({ size = "default", elevation = "sm", className, children, ...props }, ref) => (
    <Card ref={ref} elevation={elevation} className={toolbarCn({ size }, className)} {...props}>
      {children}
    </Card>
  ),
);
ToolbarRoot.displayName = "Toolbar";

/* ── Group ─────────────────────────────────────────────────── */

const ToolbarGroup = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn("flex items-center gap-1", className)}
      {...props}
    />
  ),
);
ToolbarGroup.displayName = "Toolbar.Group";

/* ── Divider ───────────────────────────────────────────────── */

const ToolbarDivider = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      role="separator"
      aria-orientation="vertical"
      className={cn("mx-1 h-6 w-px shrink-0 bg-border", className)}
      {...props}
    />
  ),
);
ToolbarDivider.displayName = "Toolbar.Divider";

/* ── Namespace ─────────────────────────────────────────────── */

export const Toolbar = Object.assign(ToolbarRoot, {
  Root: ToolbarRoot,
  Group: ToolbarGroup,
  Divider: ToolbarDivider,
});
