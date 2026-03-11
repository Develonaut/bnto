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

/** Cell size in pixels — room for grid zones (header, body, footer, edges). */
const CELL = 170;

/** Horizontal gap between cells in pixels — room for divider + add button. */
const GAP_X = 30;

/** Vertical gap below cells in pixels — room for divider + add button. */
const GAP_Y = 22;

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

/** Vertical offset from parent to children row when a container is expanded. */
const ROW_OFFSET = CELL + GAP_Y + 8;

/** Inset from the GAP_X padding for container group overlay borders.
 *  Set to GAP_X / 2 so the group border aligns with divider center lines. */
const GROUP_INSET = GAP_X / 2;

/** Divider thickness on the non-primary axis (px). */
const DIVIDER_THIN = 16;

/** Maximum nesting depth for container expansion. */
const MAX_CONTAINER_DEPTH = 3;

/** Derive child flow direction from container depth. Even = down, odd = right. */
function getChildDirection(depth: number): "vertical" | "horizontal" {
  return depth % 2 === 0 ? "vertical" : "horizontal";
}

export { CELL, GAP_X, GAP_Y, STRIDE, SLOTS, IO_CARD_SIZE, ROW_OFFSET, GROUP_INSET, DIVIDER_THIN, MAX_CONTAINER_DEPTH, getChildDirection };
