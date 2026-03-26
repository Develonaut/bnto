import type { ReactNode } from "react";

interface ThumbPositionerProps {
  offsetX: number;
  offsetY: number;
  children: ReactNode;
}

/** Absolutely-positioned wrapper that places the thumb at the computed offset. */
export function ThumbPositioner({ offsetX, offsetY, children }: ThumbPositionerProps) {
  return (
    <div
      className="pointer-events-none absolute left-1/2 top-1/2"
      style={{
        transform: `translate(calc(${offsetX}px - 50%), calc(${offsetY}px - 50%))`,
      }}
    >
      {children}
    </div>
  );
}
