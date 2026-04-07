import type { ReactNode } from "react";

import { cn } from "@bnto/ui";

interface MarqueeTrackProps {
  children: ReactNode;
  vertical: boolean;
  reverse: boolean;
  pauseOnHover: boolean;
}

export function MarqueeTrack({ children, vertical, reverse, pauseOnHover }: MarqueeTrackProps) {
  return (
    <div
      className={cn("flex shrink-0 justify-around gap-(--gap)", {
        "animate-marquee flex-row": !vertical,
        "animate-marquee-vertical flex-col": vertical,
        "group-hover:[animation-play-state:paused]": pauseOnHover,
        "[animation-direction:reverse]": reverse,
      })}
    >
      {children}
    </div>
  );
}
