"use client";

import type { ReactNode, PointerEvent, KeyboardEvent, RefObject } from "react";

import { cn } from "../../utils/cn";

interface RadialSliderContainerProps {
  containerRef: RefObject<HTMLDivElement | null>;
  onPointerDown: (e: PointerEvent) => void;
  onPointerMove: (e: PointerEvent) => void;
  onPointerUp: () => void;
  onPointerLeave: () => void;
  onKeyDown: (e: KeyboardEvent) => void;
  size: number;
  ariaLabel?: string;
  min: number;
  max: number;
  value: number;
  className?: string;
  children: ReactNode;
}

export function RadialSliderContainer(props: RadialSliderContainerProps) {
  return (
    <div
      ref={props.containerRef}
      className={cn("relative touch-none select-none focus-ring rounded-full", props.className)}
      style={{ width: props.size, height: props.size }}
      role="slider"
      tabIndex={0}
      aria-valuemin={props.min}
      aria-valuemax={props.max}
      aria-valuenow={props.value}
      aria-label={props.ariaLabel}
      onPointerDown={props.onPointerDown}
      onPointerMove={props.onPointerMove}
      onPointerUp={props.onPointerUp}
      onPointerLeave={props.onPointerLeave}
      onKeyDown={props.onKeyDown}
    >
      {props.children}
    </div>
  );
}
