"use client";

import { useRef } from "react";
import type { ReactNode } from "react";

import { cn } from "../../utils/cn";
import { GripVerticalIcon } from "../../icons";
import { Pressable } from "../../surface/Pressable";
import { Surface } from "../../surface/Surface";

import { arcPath, polarToOffset } from "./geometry";
import { valueToAngle } from "./valueMapping";
import { useRadialPointer } from "./useRadialPointer";
import { useRadialKeyboard } from "./useRadialKeyboard";
import { RadialSliderSvg } from "./RadialSliderSvg";

export interface RadialSliderProps {
  min: number;
  max: number;
  value: number;
  onChange: (value: number) => void;
  step?: number;
  startAngle?: number;
  endAngle?: number;
  size?: number;
  strokeWidth?: number;
  trackClassName?: string;
  activeClassName?: string;
  hideProgress?: boolean;
  hideRing?: boolean;
  svgDefs?: ReactNode;
  trackStroke?: string;
  activeStroke?: string;
  renderThumb?: (props: { isDragging: boolean }) => ReactNode;
  children?: ReactNode;
  "aria-label"?: string;
  className?: string;
}

export function RadialSliderRoot({
  min,
  max,
  value,
  onChange,
  step = 1,
  startAngle = 0,
  endAngle = 360,
  size = 48,
  strokeWidth = 10,
  trackClassName = "text-input",
  activeClassName = "text-primary",
  hideProgress = false,
  hideRing = false,
  svgDefs,
  trackStroke,
  activeStroke,
  renderThumb,
  children,
  "aria-label": ariaLabel,
  className,
}: RadialSliderProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const thumbRef = useRef<HTMLDivElement>(null);

  // Track radius -- inset from the edge to leave room for the thumb
  const trackInset = strokeWidth * 2 + 4;
  const radius = (size - trackInset) / 2;
  const svgCenter = size / 2;

  // Thumb position as offset from center
  const currentAngle = valueToAngle(value, min, max, startAngle, endAngle);
  const thumbOffset = polarToOffset(currentAngle, radius);

  // Active progress arc
  const activePath =
    !hideProgress && value > min
      ? arcPath(startAngle, currentAngle, radius, svgCenter, svgCenter)
      : "";

  const { isDragging, isHovering, onPointerDown, onPointerMove, onPointerUp, clearHover } =
    useRadialPointer({ containerRef, thumbRef, min, max, startAngle, endAngle, onChange });

  const onKeyDown = useRadialKeyboard({ min, max, value, step, onChange });

  const defaultThumb = (
    <Pressable asChild spring="bouncy" pressed={isDragging} hovered={isHovering}>
      <Surface asChild variant="primary" elevation="sm" rounded="full">
        <div ref={thumbRef} className="flex items-center justify-center size-8 ring-0">
          <GripVerticalIcon strokeWidth={3} className="size-3.5 shrink-0" />
        </div>
      </Surface>
    </Pressable>
  );

  return (
    <div
      ref={containerRef}
      className={cn("relative touch-none select-none outline-none", className)}
      style={{ width: size, height: size }}
      role="slider"
      tabIndex={0}
      aria-valuemin={min}
      aria-valuemax={max}
      aria-valuenow={value}
      aria-label={ariaLabel}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerLeave={clearHover}
      onKeyDown={onKeyDown}
    >
      <RadialSliderSvg
        size={size}
        radius={radius}
        svgCenter={svgCenter}
        strokeWidth={strokeWidth}
        startAngle={startAngle}
        endAngle={endAngle}
        trackClassName={trackClassName}
        activeClassName={activeClassName}
        hideRing={hideRing}
        svgDefs={svgDefs}
        trackStroke={trackStroke}
        activeStroke={activeStroke}
        activePath={activePath}
      />

      {children && (
        <div className="pointer-events-none absolute inset-0 flex items-center justify-center">
          {children}
        </div>
      )}

      <div
        className="pointer-events-none absolute left-1/2 top-1/2"
        style={{
          transform: `translate(calc(${thumbOffset.x}px - 50%), calc(${thumbOffset.y}px - 50%))`,
        }}
      >
        {renderThumb ? renderThumb({ isDragging }) : defaultThumb}
      </div>
    </div>
  );
}
