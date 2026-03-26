"use client";

import type { ReactNode } from "react";

import { useRadialSlider } from "./useRadialSlider";
import { RadialSliderSvg } from "./RadialSliderSvg";
import { RadialSliderContainer } from "./RadialSliderContainer";
import { RadialSliderOverlay } from "./RadialSliderOverlay";
import { ThumbPositioner } from "./ThumbPositioner";
import { resolveThumb } from "./ResolveThumb";

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

export function RadialSliderRoot(p: RadialSliderProps) {
  const { containerRef, thumbRef, d, layout, pointer, onKeyDown } = useRadialSlider(p);
  const thumb = resolveThumb({
    isDragging: pointer.isDragging,
    isHovering: pointer.isHovering,
    thumbRef,
    renderThumb: p.renderThumb,
  });

  return (
    <RadialSliderContainer
      containerRef={containerRef}
      onPointerDown={pointer.onPointerDown}
      onPointerMove={pointer.onPointerMove}
      onPointerUp={pointer.onPointerUp}
      onPointerLeave={pointer.clearHover}
      onKeyDown={onKeyDown}
      size={d.size}
      ariaLabel={p["aria-label"]}
      min={p.min}
      max={p.max}
      value={p.value}
      className={p.className}
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
        svgDefs={p.svgDefs}
        trackStroke={p.trackStroke}
        activeStroke={p.activeStroke}
        activePath={layout.activePath}
      />
      {p.children && <RadialSliderOverlay>{p.children}</RadialSliderOverlay>}
      <ThumbPositioner offsetX={layout.thumbOffset.x} offsetY={layout.thumbOffset.y}>
        {thumb}
      </ThumbPositioner>
    </RadialSliderContainer>
  );
}
