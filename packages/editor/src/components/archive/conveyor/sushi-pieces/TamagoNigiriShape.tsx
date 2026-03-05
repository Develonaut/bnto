/**
 * Tamago Nigiri — egg nigiri (top-down view).
 *
 * Same rice pad + nori band as salmon nigiri, but with a golden
 * yellow egg block topping and subtle grill/fold lines.
 */

import { NORI, RICE, TAMAGO, TAMAGO_GRILL } from "../sushi-colors";
import { S } from "./constants";

export function TamagoNigiriShape() {
  const w = S * 1.8;
  const h = S * 1.5;
  const sx = -w * 0.38;
  const sy = -h * 0.33;
  const sw = w * 0.76;
  const sh = h * 0.66;
  return (
    <>
      {/* Rice pad */}
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
      {/* Tamago (egg) topping */}
      <rect x={sx} y={sy} width={sw} height={sh} rx={2.5} fill={TAMAGO} />
      {/* Grill/fold lines — horizontal marks on the egg */}
      <line x1={sx + 2} y1={sy + sh * 0.35} x2={sx + sw - 2} y2={sy + sh * 0.35} stroke={TAMAGO_GRILL} strokeWidth={0.8} strokeLinecap="round" />
      <line x1={sx + 2} y1={sy + sh * 0.65} x2={sx + sw - 2} y2={sy + sh * 0.65} stroke={TAMAGO_GRILL} strokeWidth={0.8} strokeLinecap="round" />
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
