/**
 * watermarkLayout — pure positioning math for watermark preview.
 *
 * These anchor percentages MUST match the Rust engine's calculate_position()
 * in engine/crates/bnto-image/src/watermark.rs. The engine uses:
 *   top-left     → (0, 0)        → anchor at image edge
 *   center       → (sw/2, sh/2)  → anchor at image center
 *   bottom-right → (sw, sh)      → anchor at opposite edge
 *
 * The preview CSS positions the watermark's center on the same anchor using
 * transform: translate(-50%, -50%).
 */

import type { CSSProperties } from "react";

/** Anchor positions as percentages — matches engine's calculate_position(). */
const ANCHOR_PERCENT: Record<string, { x: number; y: number }> = {
  "top-left": { x: 0, y: 0 },
  "top-center": { x: 50, y: 0 },
  "top-right": { x: 100, y: 0 },
  "middle-left": { x: 0, y: 50 },
  center: { x: 50, y: 50 },
  "middle-right": { x: 100, y: 50 },
  "bottom-left": { x: 0, y: 100 },
  "bottom-center": { x: 50, y: 100 },
  "bottom-right": { x: 100, y: 100 },
};

/**
 * Compute the letterboxed image area within a 1:1 container as CSS insets.
 * A landscape image gets top/bottom bars; a portrait image gets side bars.
 */
function imageAreaStyle(ratio: number | undefined): CSSProperties {
  if (!ratio || ratio === 1) return { inset: 0 };
  if (ratio > 1) {
    const h = (1 / ratio) * 100;
    const pad = (100 - h) / 2;
    return { left: 0, right: 0, top: `${pad}%`, bottom: `${pad}%` };
  }
  const w = ratio * 100;
  const pad = (100 - w) / 2;
  return { top: 0, bottom: 0, left: `${pad}%`, right: `${pad}%` };
}

export { ANCHOR_PERCENT, imageAreaStyle };
