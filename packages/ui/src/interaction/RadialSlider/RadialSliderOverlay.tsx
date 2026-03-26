import type { ReactNode } from "react";

/** Center overlay for children content inside the radial slider. */
export function RadialSliderOverlay({ children }: { children: ReactNode }) {
  return (
    <div className="pointer-events-none absolute inset-0 flex items-center justify-center">
      {children}
    </div>
  );
}
