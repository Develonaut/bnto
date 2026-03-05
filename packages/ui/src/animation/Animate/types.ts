import type { HTMLAttributes } from "react";
import type { Easing } from "./buildStyle";

export interface AnimateBaseProps extends HTMLAttributes<HTMLDivElement> {
  asChild?: boolean;
  /** Position in a `<Animate.Stagger>` cascade. Sets `--stagger-index`. */
  index?: number;
  /** Override the animation easing. Sets `animationTimingFunction`. */
  easing?: Easing;
}

export type { Easing };
