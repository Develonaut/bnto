import type { ComponentProps } from "react";

import { cn } from "../utils/cn";

/* ── Divider ──────────────────────────────────────────────────
 * Dashed line divider with optional centered label.
 *
 *   <Divider />                          horizontal (default)
 *   <Divider orientation="vertical" />   vertical
 *   <Divider label="Free. No signup." />
 * ────────────────────────────────────────────────────────────── */

const hLineCn = [
  "h-px flex-1",
  "bg-[repeating-linear-gradient(90deg,transparent,transparent_4px,currentColor_4px,currentColor_10px)]",
];

const vLineCn = [
  "w-px flex-1",
  "bg-[repeating-linear-gradient(180deg,transparent,transparent_4px,currentColor_4px,currentColor_10px)]",
];

const hFadeCn = "[mask-image:linear-gradient(90deg,transparent,black_10%,black_90%,transparent)]";
const vFadeCn = "[mask-image:linear-gradient(180deg,transparent,black_10%,black_90%,transparent)]";

type DividerProps = ComponentProps<"div"> & {
  /** Centered label text. */
  label?: string;
  /** Orientation of the divider line. Default: "horizontal". */
  orientation?: "horizontal" | "vertical";
};

export function Divider({ label, orientation = "horizontal", className, ...props }: DividerProps) {
  const isVertical = orientation === "vertical";
  const lineCn = isVertical ? vLineCn : hLineCn;
  const fadeMask = isVertical ? vFadeCn : hFadeCn;

  return (
    <div
      className={cn(
        "text-muted-foreground/40 flex items-center gap-4",
        isVertical ? "flex-col" : "flex-row",
        className,
      )}
      role="separator"
      aria-orientation={orientation}
      {...props}
    >
      {label ? (
        <>
          <div className={cn(lineCn, fadeMask)} />
          <span className="text-foreground/65 bg-muted shrink-0 rounded-sm px-3 py-1 font-mono text-xs uppercase tracking-widest">
            {label}
          </span>
          <div className={cn(lineCn, fadeMask)} />
        </>
      ) : (
        <div className={cn(lineCn)} />
      )}
    </div>
  );
}
