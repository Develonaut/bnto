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

/** Cell size in pixels — 40px larger than IO_CARD_SIZE (100). */
const CELL = 140;

/** Horizontal gap between cells in pixels. */
const GAP_X = 16;

/** Vertical gap below cells in pixels — room for edge-positioned actions. */
const GAP_Y = 30;

/** Stride — one cell + horizontal gap. */
const STRIDE = CELL + GAP_X;

/** Max number of nodes supported. */
const MAX_NODES = 10;

/**
 * Predefined slot positions — horizontal strip, uniform size.
 * Generated from index: each node is CELL wide, spaced by GAP_X.
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

/** I/O inner card size — visually smaller, but the RF node is still CELL×CELL. */
const IO_CARD_SIZE = 100;

export { CELL, GAP_X, GAP_Y, STRIDE, SLOTS, IO_CARD_SIZE };
