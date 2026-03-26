import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@bnto/ui";

import { MarqueeTrack } from "./MarqueeTrack";

interface MarqueeProps extends ComponentPropsWithoutRef<"div"> {
  /** Whether to reverse the animation direction. */
  reverse?: boolean;
  /** Whether to pause the animation on hover. */
  pauseOnHover?: boolean;
  /** Content to be displayed in the marquee. */
  children: React.ReactNode;
  /** Whether to animate vertically instead of horizontally. */
  vertical?: boolean;
  /** Number of times to repeat the content. */
  repeat?: number;
}

export function Marquee({
  className,
  reverse = false,
  pauseOnHover = false,
  children,
  vertical = false,
  repeat = 4,
  ...props
}: MarqueeProps) {
  return (
    <div
      {...props}
      className={cn(
        "group flex gap-(--gap) overflow-hidden p-2 [--duration:40s] [--gap:1rem]",
        vertical ? "flex-col" : "flex-row",
        className,
      )}
    >
      {Array(repeat)
        .fill(0)
        .map((_, i) => (
          <MarqueeTrack key={i} vertical={vertical} reverse={reverse} pauseOnHover={pauseOnHover}>
            {children}
          </MarqueeTrack>
        ))}
    </div>
  );
}
