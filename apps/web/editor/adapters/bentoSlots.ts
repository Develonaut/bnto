/**
 * Bento grid layout constants — single source of truth.
 *
 * Shared by the editor adapter (definitionToBento) and the
 * visual showcase (BentoBoxShowcase). If the layout changes,
 * it changes here and both consumers stay in sync.
 *
 * Layout (3 rows, horizontal flow, 660px total width):
 * ┌────────┬────────────┬────────┬──────────┐
 * │   1    │     2      │   3    │    4     │
 * ├────────┴────────────┼────────┴──────────┤
 * │         5           │        6          │
 * ├──────────┬──────────┼────────┬──────────┤
 * │    7     │    8     │   9    │   10     │
 * └──────────┴──────────┴────────┴──────────┘
 */

/** Cell size in pixels. */
const CELL = 120;

/** Gap between cells in pixels. */
const GAP = 20;

/** Stride — one cell + one gap. */
const STRIDE = CELL + GAP;

/**
 * Predefined bento slot positions for up to 10 compartments.
 *
 * Column boundaries: 0, 140, 340, 480, 660
 * Left half:  0–340 (340px)   Right half: 340–660 (320px)
 * Row 0: 120 | 20 | 200 | 20 | 120 | 20 | 160 = 660
 * Row 1: 340          | 20 |          300       = 660
 * Row 2: 160 | 20 | 160 | 20 | 140 | 20 | 140 = 660
 */
const SLOTS: { x: number; y: number; w: number; h: number }[] = [
  /* Row 0 — alternating small + wide, left to right */
  { x: 0, y: 0, w: 120, h: CELL },
  { x: 140, y: 0, w: 200, h: CELL },
  { x: 360, y: 0, w: 120, h: CELL },
  { x: 500, y: 0, w: 160, h: CELL },
  /* Row 1 — two wide panels spanning each half */
  { x: 0, y: STRIDE, w: 340, h: CELL },
  { x: 360, y: STRIDE, w: 300, h: CELL },
  /* Row 2 — four cells filling the bottom */
  { x: 0, y: STRIDE * 2, w: 160, h: CELL },
  { x: 180, y: STRIDE * 2, w: 160, h: CELL },
  { x: 360, y: STRIDE * 2, w: 140, h: CELL },
  { x: 520, y: STRIDE * 2, w: 140, h: CELL },
];

export { CELL, GAP, STRIDE, SLOTS };
