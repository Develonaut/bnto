import type { ComponentProps } from "react";

import { cn } from "../utils/cn";

/* ── Divider ──────────────────────────────────────────────────
 * Dashed line divider with optional centered label.
 *
 *   <Divider />
 *   <Divider label="Free. No signup." />
 * ────────────────────────────────────────────────────────────── */

const baseCn = [
  "h-px flex-1",
  "bg-[repeating-linear-gradient(90deg,transparent,transparent_4px,currentColor_4px,currentColor_10px)]",
];

const fadeCn = "[mask-image:linear-gradient(90deg,transparent,black_10%,black_90%,transparent)]";

type DividerProps = ComponentProps<"div"> & {
  /** Centered label text. */
  label?: string;
};

export function Divider({ label, className, ...props }: DividerProps) {
  return (
    <div
      className={cn("text-muted-foreground/40 flex items-center gap-4", className)}
      role="separator"
      {...props}
    >
      {label ? (
        <>
          <div className={cn(baseCn, fadeCn)} />
          <span className="text-foreground/65 bg-muted shrink-0 rounded-sm px-3 py-1 font-mono text-xs uppercase tracking-widest">
            {label}
          </span>
          <div className={cn(baseCn, fadeCn)} />
        </>
      ) : (
        <div className={cn(baseCn)} />
      )}
    </div>
  );
}
