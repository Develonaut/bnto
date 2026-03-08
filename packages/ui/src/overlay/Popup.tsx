"use client";

import type { CSSProperties, ReactNode } from "react";

import { ScaleIn } from "../animation/Animate";
import type { Easing } from "../animation/Animate/types";
import { Card } from "../surface/Card";
import type { SurfaceElevation } from "../surface/Surface";

type Side = "top" | "bottom" | "left" | "right";

/** Map placement side → ScaleIn origin so the popup scales from the edge nearest the trigger. */
const SIDE_ORIGIN = {
  top: "bottom",
  bottom: "top",
  left: "right",
  right: "left",
} as const;

interface PopupProps {
  /** Placement side — sets transform origin to the opposite edge. */
  side?: Side;
  /** Card elevation. Default "lg". */
  elevation?: SurfaceElevation;
  /** ScaleIn starting scale (0–1). Default 0.6. */
  from?: number;
  /** Spring easing. Default "spring-bouncier". */
  easing?: Easing;
  /** Raw CSS transform-origin override (e.g. Radix CSS vars). */
  originStyle?: string;
  className?: string;
  style?: CSSProperties;
  children: ReactNode;
}

export function Popup({
  side,
  elevation = "lg",
  from = 0.6,
  easing = "spring-bouncier",
  originStyle,
  className,
  style,
  children,
}: PopupProps) {
  return (
    <ScaleIn
      from={from}
      origin={side ? SIDE_ORIGIN[side] : undefined}
      easing={easing}
      style={originStyle ? { ...style, transformOrigin: originStyle } : style}
    >
      <Card className={className} elevation={elevation}>
        {children}
      </Card>
    </ScaleIn>
  );
}
