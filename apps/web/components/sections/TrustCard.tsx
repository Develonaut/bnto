"use client";

import type { ReactNode } from "react";
import { Card, Stack } from "@bnto/ui";

import { useStepInView } from "./useStepInView";

interface TrustCardProps {
  mascot?: string;
  mascotSize?: number;
  footer?: ReactNode;
  children: ReactNode;
}

export function TrustCard({
  mascot = "/mascots/sumo-sushi.svg",
  mascotSize = 260,
  footer,
  children,
}: TrustCardProps) {
  const [inView, ref] = useStepInView(0.3);

  return (
    <Card ref={ref} dormant={!inView} elevation="lg" className="w-full p-6">
      <Stack className="gap-5">
        <div className="grid items-center gap-6 sm:grid-cols-[auto_1fr]">
          {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
          <img
            src={mascot}
            alt=""
            width={mascotSize}
            height={mascotSize}
            className="mx-auto shrink-0"
            aria-hidden
          />
          {children}
        </div>
        {footer && <div className="border-t border-border pt-4">{footer}</div>}
      </Stack>
    </Card>
  );
}
