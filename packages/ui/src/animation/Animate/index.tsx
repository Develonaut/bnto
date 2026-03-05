import type { ReactNode } from "react";

import { Stagger } from "./Stagger";
import { ScaleIn } from "./ScaleIn";
import { FadeIn } from "./FadeIn";
import { SlideUp } from "./SlideUp";
import { SlideDown } from "./SlideDown";
import { PulseSoft } from "./PulseSoft";
import { Breathe } from "./Breathe";
import { BouncyStagger } from "./BouncyStagger";
import { InView } from "../InView";

function AnimateRoot({ children }: { children: ReactNode }) {
  return <>{children}</>;
}

export const Animate = Object.assign(AnimateRoot, {
  Root: AnimateRoot,
  Stagger,
  ScaleIn,
  BouncyStagger,
  FadeIn,
  SlideUp,
  SlideDown,
  PulseSoft,
  Breathe,
  InView,
});

/* Re-export for direct import from server components.
 * Server components can't access Object.assign namespace properties
 * on client references -- use named imports instead of
 * `Animate.X` when rendering from a server component. */
export { InView, BouncyStagger };
