/**
 * DirectionConfig — axis-specific config for container divider injection.
 *
 * Encapsulates the differences between vertical (even depth) and
 * horizontal (odd depth) child flow so divider helpers don't need
 * to branch on direction themselves.
 */

import type { BentoNode } from "../../../adapters/types";
import { CELL, STRIDE, ROW_OFFSET, DIVIDER_THIN, getChildDirection } from "../../../adapters/bentoSlots";

interface DirectionConfig {
  /** Divider label passed to AddDividerNode renderer. */
  direction: "vertical" | "horizontal";
  /** Divider width (thin for horizontal flow, full cell for vertical flow). */
  width: number;
  /** Divider height (full cell for horizontal flow, thin for vertical flow). */
  height: number;
  /** Gap between nodes on the primary axis. */
  gap: number;
  /** Read primary-axis position from a node. */
  primary: (n: BentoNode) => number;
  /** Build position for a divider given primary-axis value. */
  pos: (primaryVal: number, parent: BentoNode) => { x: number; y: number };
  /** Sort comparator for children along primary axis. */
  sort: (a: BentoNode, b: BentoNode) => number;
}

function verticalConfig(): DirectionConfig {
  return {
    direction: "vertical",
    width: CELL, height: DIVIDER_THIN,
    gap: ROW_OFFSET,
    primary: (n) => n.position.y,
    pos: (y, parent) => ({ x: parent.position.x, y }),
    sort: (a, b) => a.position.y - b.position.y,
  };
}

function horizontalConfig(): DirectionConfig {
  return {
    direction: "horizontal",
    width: DIVIDER_THIN, height: CELL,
    gap: STRIDE,
    primary: (n) => n.position.x,
    pos: (x, parent) => ({ x, y: parent.position.y }),
    sort: (a, b) => a.position.x - b.position.x,
  };
}

/** Resolve direction config from a container's depth. */
function directionConfigFor(parent: BentoNode): DirectionConfig {
  const depth = parent.data.depth ?? 0;
  return getChildDirection(depth) === "vertical" ? verticalConfig() : horizontalConfig();
}

export { directionConfigFor };
export type { DirectionConfig };
