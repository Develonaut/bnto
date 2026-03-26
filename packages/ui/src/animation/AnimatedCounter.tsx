"use client";

import { useEffect, useRef } from "react";

import { runCountAnimation } from "./runCountAnimation";

interface AnimatedCounterProps {
  /** Target value to count to */
  value: number;
  /** Whether the animation is active (typically tied to visibility) */
  active: boolean;
  /** Animation duration in ms */
  duration?: number;
  /** Optional suffix rendered inline (e.g. "KB", "%") */
  suffix?: string;
  /** Optional className for the suffix */
  suffixClassName?: string;
  /** Optional className for the wrapper */
  className?: string;
}

/**
 * Animated number counter that counts from 0 to a target value
 * with an ease-out cubic curve when `active` becomes true.
 *
 * Uses direct DOM writes (`textContent`) instead of React state
 * to avoid re-renders on every animation frame.
 */
export function AnimatedCounter({
  value,
  active,
  duration = 1200,
  suffix,
  suffixClassName,
  className,
}: AnimatedCounterProps) {
  const numberRef = useRef<HTMLSpanElement>(null);

  useEffect(() => {
    const el = numberRef.current;
    if (!el) return;
    if (!active) {
      el.textContent = "0";
      return;
    }
    return runCountAnimation(el, value, duration);
  }, [active, value, duration]);

  return (
    <span className={className}>
      <span ref={numberRef}>0</span>
      {suffix && <span className={suffixClassName}>{suffix}</span>}
    </span>
  );
}
