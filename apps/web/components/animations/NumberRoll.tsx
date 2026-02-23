"use client";

import { useEffect, useRef } from "react";
import { motion, useSpring, useTransform, useReducedMotion } from "motion/react";

import { springs } from "./config";

interface NumberRollProps {
  /** The target number to display. */
  value: number;
  /** Number of decimal places. */
  decimals?: number;
  /** Format function for display (e.g., adding units). */
  format?: (value: number) => string;
  className?: string;
}

/**
 * Score incrementing.
 *
 * Animated number transitions — the value rolls smoothly from
 * its previous value to the new one. Uses spring physics.
 */
export function NumberRoll({
  value,
  decimals = 0,
  format,
  className,
}: NumberRollProps) {
  const prefersReduced = useReducedMotion();
  const springValue = useSpring(0, springs.smooth);
  const display = useTransform(springValue, (v) => {
    const rounded = decimals > 0 ? v.toFixed(decimals) : Math.round(v).toString();
    return format ? format(Number(rounded)) : rounded;
  });
  const initialized = useRef(false);

  useEffect(() => {
    if (prefersReduced || !initialized.current) {
      springValue.jump(value);
      initialized.current = true;
    } else {
      springValue.set(value);
    }
  }, [value, springValue, prefersReduced]);

  return <motion.span className={className}>{display}</motion.span>;
}
