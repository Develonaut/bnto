import type { ComponentProps } from "react";

import { createCn } from "../utils/createCn";

const kbdCn = createCn({
  base: "inline-flex items-center justify-center rounded-md border border-border bg-muted font-mono text-muted-foreground leading-none",
  variants: {
    size: {
      sm: "min-w-5 h-5 px-1 text-[10px]",
      md: "min-w-6 h-6 px-1.5 text-xs",
    },
  },
  defaultVariants: {
    size: "sm",
  },
});

type KbdSize = "sm" | "md";

type KbdProps = ComponentProps<"kbd"> & {
  size?: KbdSize;
};

/** Renders a single keyboard key with styled <kbd> element. */
export function Kbd({ size, className, ...props }: KbdProps) {
  return <kbd className={kbdCn({ size }, className)} {...props} />;
}

type KbdGroupProps = ComponentProps<"span">;

/** Groups multiple Kbd elements with tight spacing. */
export function KbdGroup({ className, ...props }: KbdGroupProps) {
  return <span className={`inline-flex items-center gap-0.5 ${className ?? ""}`} {...props} />;
}
