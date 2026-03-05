/**
 * Bento grid layout constants — single source of truth.
 *
 * Simple horizontal strip: nodes placed left-to-right with
 * uniform size. Keeps fitView predictable when adding/removing.
 *
 * Layout (single row, horizontal flow):
 * ┌────────┬────────┬────────┬────────┬─ ─ ─ ─┐
 * │   1    │   2    │   3    │   4    │  ...   │
 * └────────┴────────┴────────┴────────┴─ ─ ─ ─┘
 */

/** Cell size in pixels. */
const CELL = 200;

/** Gap between cells in pixels. */
const GAP = 20;

/** Stride — one cell + one gap. */
const STRIDE = CELL + GAP;

/** Max number of nodes supported. */
const MAX_NODES = 10;

/**
 * Predefined slot positions — horizontal strip, uniform size.
 * Generated from index: each node is CELL wide, spaced by GAP.
 */
const SLOTS: { x: number; y: number; w: number; h: number }[] = Array.from(
  { length: MAX_NODES },
  (_, i) => ({
    x: i * STRIDE,
    y: 0,
    w: CELL,
    h: CELL,
  }),
);

export { CELL, GAP, STRIDE, SLOTS };
