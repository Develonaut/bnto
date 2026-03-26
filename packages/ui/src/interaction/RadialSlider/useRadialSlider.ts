"use client";

import { useRef } from "react";

import { useRadialPointer } from "./useRadialPointer";
import { useRadialKeyboard } from "./useRadialKeyboard";
import { computeRadialLayout } from "./useRadialLayout";
import { resolveRadialDefaults } from "./resolveRadialDefaults";
import type { RadialSliderProps } from "./RadialSliderRoot";

/** Orchestrate refs, layout computation, pointer/keyboard hooks for the radial slider. */
export function useRadialSlider(props: RadialSliderProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const thumbRef = useRef<HTMLDivElement>(null);
  const d = resolveRadialDefaults(props);
  const { min, max, value, onChange } = props;
  const layout = computeRadialLayout({
    size: d.size, strokeWidth: d.strokeWidth, min, max, value,
    startAngle: d.startAngle, endAngle: d.endAngle, hideProgress: d.hideProgress,
  });
  const pointer = useRadialPointer({
    containerRef, thumbRef, min, max,
    startAngle: d.startAngle, endAngle: d.endAngle, onChange,
  });
  const onKeyDown = useRadialKeyboard({ min, max, value, step: d.step, onChange });
  return { containerRef, thumbRef, d, layout, pointer, onKeyDown };
}
