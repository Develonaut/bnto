"use client";

import { Children, createContext, forwardRef, isValidElement, useContext } from "react";
import type { ComponentProps, HTMLAttributes, ReactNode } from "react";

import { cn } from "../utils/cn";

import { Button } from "../interaction/Button";
import { Card } from "./Card";
import { PanelLeftIcon, PanelLeftCloseIcon } from "../icons";

/**
 * Panel — floating Card surface with optional collapse behavior.
 *
 * Always-expanded by default. When `collapsed` + `onToggle` are provided,
 * collapses to a compact pill (rounded-full) with a single toggle button.
 * Spring-animated width transition. Used for editor sidebar, config panels.
 *
 * Usage (always expanded):
 *   <Panel className="h-full w-full">
 *     <Panel.Header><Text>Title</Text></Panel.Header>
 *     <Panel.Content><NodeList /></Panel.Content>
 *   </Panel>
 *
 * Usage (collapsible):
 *   <Panel collapsed={collapsed} onToggle={toggle} className="w-56">
 *     <Panel.Header>
 *       <Panel.Trigger />
 *       <Text>Title</Text>
 *     </Panel.Header>
 *     <Panel.Content><NodeList /></Panel.Content>
 *   </Panel>
 */

/* ── Context ──────────────────────────────────────────────── */

interface PanelContextValue {
  collapsed: boolean;
  onToggle?: () => void;
}

const PanelContext = createContext<PanelContextValue | null>(null);

function usePanelContext() {
  const ctx = useContext(PanelContext);
  if (!ctx) throw new Error("Panel sub-components must be used within <Panel>");
  return ctx;
}

/* ── Root ──────────────────────────────────────────────────── */

interface PanelRootProps extends Omit<ComponentProps<typeof Card>, "children"> {
  collapsed?: boolean;
  onToggle?: () => void;
  children: ReactNode;
}

/* CSS property transition (not an entrance animation) — Animate.* components
   handle keyframe entrances; property transitions on width/padding are the
   correct CSS-first approach per the animation decision tree. */
const PANEL_BASE =
  "flex h-full flex-col motion-safe:transition-[width,padding] motion-safe:duration-slow motion-safe:ease-spring-bouncy";

const PanelRoot = forwardRef<HTMLDivElement, PanelRootProps>(
  ({ collapsed = false, onToggle, elevation = "md", className, children, ...props }, ref) => (
    <PanelContext.Provider value={{ collapsed, onToggle }}>
      <Card
        ref={ref}
        elevation={elevation}
        data-collapsed={collapsed}
        className={cn(
          PANEL_BASE,
          className,
          /* Collapsed overrides come LAST so twMerge picks w-[62px] over
             any width in className (e.g. w-56). */
          collapsed && "w-[62px] rounded-full px-3 py-0",
        )}
        {...props}
      >
        {children}
      </Card>
    </PanelContext.Provider>
  ),
);
PanelRoot.displayName = "Panel";

/* ── Header ───────────────────────────────────────────────── */

const PanelHeader = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, children, ...props }, ref) => {
    const { collapsed } = usePanelContext();

    /* When collapsed, only render PanelTrigger children — everything
       else (title, count, icons) is hidden so the pill shows just the button.
       Note: Panel.Trigger must be a direct child (not wrapped in Fragment
       or another component) for the type check to work. */
    const visibleChildren = collapsed
      ? Children.toArray(children).filter(
          (child) => isValidElement(child) && child.type === PanelTrigger,
        )
      : children;

    return (
      <div
        ref={ref}
        className={cn(
          "flex shrink-0 items-center",
          className,
          /* Collapsed overrides come LAST — strip horizontal padding/border.
             Header keeps its own vertical padding for consistent trigger position.
             overflow-hidden only when expanded (clips during width transition).
             Collapsed needs visible overflow so button shadow isn't clipped. */
          collapsed
            ? "justify-center overflow-visible border-0 px-0"
            : "overflow-hidden",
        )}
        {...props}
      >
        {visibleChildren}
      </div>
    );
  },
);
PanelHeader.displayName = "Panel.Header";

/* ── Trigger ──────────────────────────────────────────────── */

const PanelTrigger = forwardRef<
  HTMLButtonElement,
  Omit<ComponentProps<typeof Button>, "onClick" | "children">
>(({ className, ...props }, ref) => {
  const { collapsed, onToggle } = usePanelContext();

  return (
    <Button
      ref={ref}
      size="icon"
      variant="ghost"
      elevation="sm"
      onClick={onToggle}
      aria-label={collapsed ? "Expand panel" : "Collapse panel"}
      className={className}
      {...props}
    >
      {collapsed ? (
        <PanelLeftIcon className="size-4" />
      ) : (
        <PanelLeftCloseIcon className="size-4" />
      )}
    </Button>
  );
});
PanelTrigger.displayName = "Panel.Trigger";

/* ── Divider ──────────────────────────────────────────────── */

const PanelDivider = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      role="separator"
      aria-orientation="horizontal"
      className={cn("my-1 h-px w-full shrink-0 bg-border", className)}
      {...props}
    />
  ),
);
PanelDivider.displayName = "Panel.Divider";

/* ── Content ──────────────────────────────────────────────── */

const PanelContent = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, children, ...props }, ref) => {
    const { collapsed } = usePanelContext();

    if (collapsed) return null;

    return (
      <div
        ref={ref}
        className={cn("flex-1 overflow-y-auto", className)}
        {...props}
      >
        {children}
      </div>
    );
  },
);
PanelContent.displayName = "Panel.Content";

/* ── Footer ───────────────────────────────────────────────── */

const PanelFooter = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, children, ...props }, ref) => {
    const { collapsed } = usePanelContext();

    if (collapsed) return null;

    return (
      <div
        ref={ref}
        className={cn("shrink-0 px-2 py-2", className)}
        {...props}
      >
        {children}
      </div>
    );
  },
);
PanelFooter.displayName = "Panel.Footer";

/* ── Namespace ────────────────────────────────────────────── */

export const Panel = Object.assign(PanelRoot, {
  Root: PanelRoot,
  Header: PanelHeader,
  Trigger: PanelTrigger,
  Divider: PanelDivider,
  Content: PanelContent,
  Footer: PanelFooter,
});
