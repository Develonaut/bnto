import type { HTMLAttributes, ReactNode } from "react";
import { cn } from "@/lib/cn";

/**
 * EditorOverlay — floating panel layer over the canvas.
 *
 * pointer-events-none container. Slots:
 *   LeftPanel   — slides in from left (Layers)
 *   RightPanel  — slides in from right (Properties)
 *   Toolbar     — bottom-center pill bar (home base)
 *
 * Toolbar buttons toggle the side panels. Panels use CSS
 * translate + spring easing for the slide-in entrance.
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

/* ── LeftPanel slot — slides in from left ────────────────── */

interface SidePanelSlotProps {
  visible?: boolean;
  children: ReactNode;
  className?: string;
}

function OverlayLeftPanel({
  visible,
  children,
  className,
}: SidePanelSlotProps) {
  return (
    <div
      onPointerDownCapture={(e) => e.stopPropagation()}
      className={cn(
        "pointer-events-auto absolute bottom-0 left-0 top-0 w-56",
        "motion-safe:transition-[translate,opacity]",
        visible
          ? "translate-x-0 opacity-100 motion-safe:duration-slow motion-safe:ease-spring-bouncy"
          : "pointer-events-none -translate-x-[110%] opacity-0 motion-safe:duration-fast motion-safe:ease-out",
        className,
      )}
    >
      {children}
    </div>
  );
}

/* ── RightPanel slot — slides in from right ──────────────── */

function OverlayRightPanel({
  visible,
  children,
  className,
}: SidePanelSlotProps) {
  return (
    <div
      onPointerDownCapture={(e) => e.stopPropagation()}
      className={cn(
        "pointer-events-auto absolute bottom-0 right-0 top-0 w-72",
        "motion-safe:transition-[translate,opacity]",
        visible
          ? "translate-x-0 opacity-100 motion-safe:duration-slow motion-safe:ease-spring-bouncy"
          : "pointer-events-none translate-x-[110%] opacity-0 motion-safe:duration-fast motion-safe:ease-out",
        className,
      )}
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
  LeftPanel: OverlayLeftPanel,
  RightPanel: OverlayRightPanel,
  Toolbar: OverlayToolbar,
});
