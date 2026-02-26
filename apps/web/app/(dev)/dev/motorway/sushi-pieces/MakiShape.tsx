/**
 * Maki — cross-section of a sushi roll.
 *
 * Three concentric circles with configurable filling color:
 *   1. Dark nori wrap (outer)
 *   2. White rice (middle)
 *   3. Colored filling (center) — with optional marbling/texture line
 *
 * Used by all maki variants (tekka, sake, ebi, hotate, tamago, kappa)
 * — the shape is identical, only the fill color changes.
 */

import { NORI, RICE, RICE_STROKE } from "../sushi-colors";
import { S } from "./constants";

export function MakiShape({ fill, line }: { fill: string; line?: string }) {
  return (
    <>
      <circle r={S} fill={NORI} stroke="#fff" strokeWidth={1.5} paintOrder="stroke" />
      <circle r={S * 0.75} fill={RICE} stroke={RICE_STROKE} strokeWidth={0.5} />
      <circle r={S * 0.4} fill={fill} />
      {/* Optional marbling/texture line through the filling */}
      {line && (
        <line
          x1={-S * 0.28}
          y1={S * 0.1}
          x2={S * 0.28}
          y2={-S * 0.1}
          stroke={line}
          strokeWidth={1.2}
          strokeLinecap="round"
        />
      )}
    </>
  );
}
