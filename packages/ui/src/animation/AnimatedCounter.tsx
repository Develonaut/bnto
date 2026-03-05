"use client";

import { useEffect, useRef } from "react";

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
 * to avoid re-renders on every animation frame. The RAF loop is
 * unavoidable -- CSS cannot natively display a changing integer --
 * but removing `useState` means zero React reconciliation during
 * the animation (typically ~60-72 frames over 1.2 s).
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
  const rafRef = useRef<number | null>(null);

  useEffect(() => {
    const el = numberRef.current;
    if (!el) return;

    const prefersReduced =
      typeof window !== "undefined" &&
      window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    // Reset to 0 when inactive
    if (!active) {
      el.textContent = "0";
      return;
    }

    // Reduced motion: jump straight to final value
    if (prefersReduced) {
      el.textContent = String(value);
      return;
    }

    const start = performance.now();

    const tick = (now: number) => {
      const elapsed = now - start;
      const progress = Math.min(elapsed / duration, 1);
      // Ease-out cubic: fast start, gentle settle
      const eased = 1 - Math.pow(1 - progress, 3);
      el.textContent = String(Math.round(eased * value));
      if (progress < 1) {
        rafRef.current = requestAnimationFrame(tick);
      }
    };

    rafRef.current = requestAnimationFrame(tick);

    return () => {
      if (rafRef.current) cancelAnimationFrame(rafRef.current);
    };
  }, [active, value, duration]);

  return (
    <span className={className}>
      <span ref={numberRef}>0</span>
      {suffix && <span className={suffixClassName}>{suffix}</span>}
    </span>
  );
}
