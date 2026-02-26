/**
 * Onigiri — rice ball with nori wrap at the base.
 *
 * Traced from Ryan's onigiri SVG reference (rice-group-onigiri 1.svg).
 * Wide rounded body with a smooth dome top — proportions match the
 * classic onigiri emoji/icon shape.
 */

import { NORI, RICE } from "../sushi-colors";
import { S } from "./constants";

export function OnigiriShape() {
  const path = [
    `M ${S * -0.25},${S * -0.84}`,
    `C ${S * -0.12},${S * -1.05} ${S * 0.12},${S * -1.05} ${S * 0.25},${S * -0.84}`,
    `C ${S * 0.6},${S * -0.35} ${S * 1.1},${S * 0.2} ${S * 1.05},${S * 0.55}`,
    `C ${S * 1.0},${S * 0.82} ${S * 0.7},${S * 0.92} ${S * 0.01},${S * 0.95}`,
    `C ${S * -0.7},${S * 0.92} ${S * -1.0},${S * 0.82} ${S * -1.05},${S * 0.55}`,
    `C ${S * -1.1},${S * 0.2} ${S * -0.6},${S * -0.35} ${S * -0.25},${S * -0.84}`,
    `Z`,
  ].join(" ");
  const noriW = 12;
  const noriH = S * 0.7;
  const noriY = S * 0.15;
  return (
    <>
      {/* Rice body — white outline via paint-order for belt contrast */}
      <path
        d={path}
        fill={RICE}
        stroke="#fff"
        strokeWidth={1.5}
        strokeLinejoin="round"
        paintOrder="stroke"
      />
      {/* Nori wrap — wide dark band with rounded top corners */}
      <rect
        x={-noriW / 2}
        y={noriY}
        width={noriW}
        height={noriH}
        rx={2.5}
        fill={NORI}
      />
    </>
  );
}
