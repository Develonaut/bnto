"use client";

import { useRef } from "react";
import type { ReactNode } from "react";

import { cn } from "../../utils/cn";

import { useRadialPointer } from "./useRadialPointer";
import { useRadialKeyboard } from "./useRadialKeyboard";
import { RadialSliderSvg } from "./RadialSliderSvg";
import { RadialSliderThumb } from "./RadialSliderThumb";
import { computeRadialLayout } from "./useRadialLayout";
import { resolveRadialDefaults } from "./resolveRadialDefaults";

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

export function RadialSliderRoot(props: RadialSliderProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const thumbRef = useRef<HTMLDivElement>(null);
  const d = resolveRadialDefaults(props);

  const layout = computeRadialLayout({
    size: d.size,
    strokeWidth: d.strokeWidth,
    min: props.min,
    max: props.max,
    value: props.value,
    startAngle: d.startAngle,
    endAngle: d.endAngle,
    hideProgress: d.hideProgress,
  });
  const pointer = useRadialPointer({
    containerRef,
    thumbRef,
    min: props.min,
    max: props.max,
    startAngle: d.startAngle,
    endAngle: d.endAngle,
    onChange: props.onChange,
  });
  const onKeyDown = useRadialKeyboard({
    min: props.min,
    max: props.max,
    value: props.value,
    step: d.step,
    onChange: props.onChange,
  });
  const thumb = resolveThumb(props, pointer, thumbRef);

  return (
    <RadialSliderContainer
      containerRef={containerRef}
      pointer={pointer}
      onKeyDown={onKeyDown}
      size={d.size}
      ariaLabel={props["aria-label"]}
      min={props.min}
      max={props.max}
      value={props.value}
      className={props.className}
    >
      <RadialSliderSvg
        size={d.size}
        radius={layout.radius}
        svgCenter={layout.svgCenter}
        strokeWidth={d.strokeWidth}
        startAngle={d.startAngle}
        endAngle={d.endAngle}
        trackClassName={d.trackClassName}
        activeClassName={d.activeClassName}
        hideRing={d.hideRing}
        svgDefs={props.svgDefs}
        trackStroke={props.trackStroke}
        activeStroke={props.activeStroke}
        activePath={layout.activePath}
      />
      {props.children && (
        <div className="pointer-events-none absolute inset-0 flex items-center justify-center">
          {props.children}
        </div>
      )}
      <div
        className="pointer-events-none absolute left-1/2 top-1/2"
        style={{
          transform: `translate(calc(${layout.thumbOffset.x}px - 50%), calc(${layout.thumbOffset.y}px - 50%))`,
        }}
      >
        {thumb}
      </div>
    </RadialSliderContainer>
  );
}

function resolveThumb(
  props: RadialSliderProps,
  pointer: ReturnType<typeof useRadialPointer>,
  thumbRef: React.RefObject<HTMLDivElement | null>,
) {
  if (props.renderThumb) return props.renderThumb({ isDragging: pointer.isDragging });
  return (
    <RadialSliderThumb
      isDragging={pointer.isDragging}
      isHovering={pointer.isHovering}
      thumbRef={thumbRef}
    />
  );
}

function RadialSliderContainer({
  containerRef,
  pointer,
  onKeyDown,
  size,
  ariaLabel,
  min,
  max,
  value,
  className,
  children,
}: {
  containerRef: React.RefObject<HTMLDivElement | null>;
  pointer: ReturnType<typeof useRadialPointer>;
  onKeyDown: (e: React.KeyboardEvent) => void;
  size: number;
  ariaLabel?: string;
  min: number;
  max: number;
  value: number;
  className?: string;
  children: ReactNode;
}) {
  return (
    <div
      ref={containerRef}
      className={cn("relative touch-none select-none focus-ring rounded-full", className)}
      style={{ width: size, height: size }}
      role="slider"
      tabIndex={0}
      aria-valuemin={min}
      aria-valuemax={max}
      aria-valuenow={value}
      aria-label={ariaLabel}
      onPointerDown={pointer.onPointerDown}
      onPointerMove={pointer.onPointerMove}
      onPointerUp={pointer.onPointerUp}
      onPointerLeave={pointer.clearHover}
      onKeyDown={onKeyDown}
    >
      {children}
    </div>
  );
}
