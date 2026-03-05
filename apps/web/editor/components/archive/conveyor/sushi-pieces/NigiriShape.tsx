/**
 * Nigiri — salmon nigiri (top-down view).
 *
 * Square-ish rice pad with rounded corners:
 *   1. White rice pad (rounded square base)
 *   2. Salmon topping (slightly smaller, overlaid)
 *   3. Salmon marbling lines (diagonal, clipped to salmon shape)
 *   4. Nori band across the middle
 */

import { NORI, RICE, SALMON, SALMON_LINE } from "../sushi-colors";
import { S } from "./constants";

export function NigiriShape() {
  const w = S * 1.8;
  const h = S * 1.5;
  const sx = -w * 0.38;
  const sy = -h * 0.33;
  const sw = w * 0.76;
  const sh = h * 0.66;
  return (
    <>
      {/* Rice pad — white outline via paint-order for belt contrast */}
      <rect
        x={-w / 2}
        y={-h / 2}
        width={w}
        height={h}
        rx={3}
        fill={RICE}
        stroke="#fff"
        strokeWidth={1.5}
        paintOrder="stroke"
      />
      {/* Salmon topping */}
      <rect x={sx} y={sy} width={sw} height={sh} rx={2.5} fill={SALMON} />
      {/* Salmon marbling — diagonal lines clipped to topping shape */}
      <g clipPath="url(#salmon-clip)">
        <line x1={sx - 2} y1={sy + sh * 0.3} x2={sx + sw + 2} y2={sy - sh * 0.2} stroke={SALMON_LINE} strokeWidth={1} />
        <line x1={sx - 2} y1={sy + sh * 0.65} x2={sx + sw + 2} y2={sy + sh * 0.15} stroke={SALMON_LINE} strokeWidth={1} />
        <line x1={sx - 2} y1={sy + sh} x2={sx + sw + 2} y2={sy + sh * 0.5} stroke={SALMON_LINE} strokeWidth={1} />
      </g>
      {/* Nori band */}
      <rect
        x={-w / 2 - 0.5}
        y={-1.5}
        width={w + 1}
        height={3}
        rx={0.5}
        fill={NORI}
      />
    </>
  );
}
