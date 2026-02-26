"use client";

import { useEffect, useRef, useState } from "react";

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
 */
export function AnimatedCounter({
  value,
  active,
  duration = 1200,
  suffix,
  suffixClassName,
  className,
}: AnimatedCounterProps) {
  const [display, setDisplay] = useState(0);
  const raf = useRef<number | null>(null);

  useEffect(() => {
    const prefersReduced =
      typeof window !== "undefined" &&
      window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    if (!active) {
      const timer = setTimeout(() => setDisplay(0), 0);
      return () => clearTimeout(timer);
    }

    if (prefersReduced) {
      const timer = setTimeout(() => setDisplay(value), 0);
      return () => clearTimeout(timer);
    }

    const start = performance.now();

    const tick = (now: number) => {
      const elapsed = now - start;
      const progress = Math.min(elapsed / duration, 1);
      // Ease-out cubic
      const eased = 1 - Math.pow(1 - progress, 3);
      setDisplay(Math.round(eased * value));
      if (progress < 1) {
        raf.current = requestAnimationFrame(tick);
      }
    };

    raf.current = requestAnimationFrame(tick);
    return () => {
      if (raf.current) cancelAnimationFrame(raf.current);
    };
  }, [active, value, duration]);

  return (
    <span className={className}>
      {display}
      {suffix && <span className={suffixClassName}>{suffix}</span>}
    </span>
  );
}
