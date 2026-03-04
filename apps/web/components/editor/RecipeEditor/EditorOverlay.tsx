import type { HTMLAttributes } from "react";
import { cn } from "@/lib/cn";

/**
 * EditorOverlay — floating panel layer over the canvas.
 *
 * pointer-events-none container with consistent inset padding.
 * Children position themselves via the slot sub-components:
 *   Sidebar  — left column, full height
 *   ConfigPanel — right column, slides in/out based on selection
 *   Toolbar  — bottom-center
 *
 * The overlay uses inset-4 for left/right/bottom padding and
 * top-[6rem] for navbar clearance (72px navbar + 24px gap).
 */

function EditorOverlayRoot({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "pointer-events-none absolute inset-4 top-[6rem] z-10",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

/* ── Sidebar slot — left column, full height ─────────────── */

function OverlaySidebar({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "pointer-events-auto absolute bottom-0 left-0 top-0",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

/* ── ConfigPanel slot — right column, slide in/out ───────── */

interface ConfigPanelSlotProps extends HTMLAttributes<HTMLDivElement> {
  /** Panel is visible when true. */
  visible?: boolean;
}

/**
 * Config panel slides in when visible, stays put when switching
 * between nodes, and only slides out on true deselection.
 *
 * CSS transition-delay on hide absorbs brief null gaps during
 * node switches (old deselect → new select in one frame). No
 * JS timers needed — pure CSS.
 */
function OverlayConfigPanel({
  visible,
  className,
  children,
  ...props
}: ConfigPanelSlotProps) {
  return (
    <div
      /* Stop pointer events from reaching ReactFlow's pane behind
       * the overlay — prevents input interactions from deselecting
       * the active node on the canvas. */
      onPointerDownCapture={(e) => e.stopPropagation()}
      className={cn(
        "pointer-events-auto absolute bottom-0 right-0 top-0 w-72 motion-safe:transition-[translate,opacity]",
        visible
          ? "translate-x-0 opacity-100 delay-0 motion-safe:duration-slow motion-safe:ease-spring-bouncy"
          : "pointer-events-none translate-x-[110%] opacity-0 delay-150 motion-safe:duration-fast motion-safe:ease-out",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

/* ── Toolbar slot — bottom-center ────────────────────────── */

function OverlayToolbar({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

/* ── Namespace ───────────────────────────────────────────── */

export const EditorOverlay = Object.assign(EditorOverlayRoot, {
  Root: EditorOverlayRoot,
  Sidebar: OverlaySidebar,
  ConfigPanel: OverlayConfigPanel,
  Toolbar: OverlayToolbar,
});
