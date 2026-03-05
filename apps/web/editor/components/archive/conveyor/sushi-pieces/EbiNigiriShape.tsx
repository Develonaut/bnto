/**
 * Ebi Nigiri — shrimp nigiri (top-down view).
 *
 * Same rice pad + nori band structure. The shrimp topping is
 * pink-orange with curved segment stripes and a darker tail tip.
 */

import { NORI, RICE, EBI, EBI_STRIPE, EBI_TAIL } from "../sushi-colors";
import { S } from "./constants";

export function EbiNigiriShape() {
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
      {/* Shrimp body */}
      <rect x={sx} y={sy} width={sw} height={sh} rx={2.5} fill={EBI} />
      {/* Shrimp segment stripes — slightly curved horizontal lines */}
      <g clipPath="url(#salmon-clip)">
        <line x1={sx + 1} y1={sy + sh * 0.25} x2={sx + sw - 1} y2={sy + sh * 0.3} stroke={EBI_STRIPE} strokeWidth={1} strokeLinecap="round" />
        <line x1={sx + 1} y1={sy + sh * 0.5} x2={sx + sw - 1} y2={sy + sh * 0.55} stroke={EBI_STRIPE} strokeWidth={1} strokeLinecap="round" />
        <line x1={sx + 1} y1={sy + sh * 0.75} x2={sx + sw - 1} y2={sy + sh * 0.8} stroke={EBI_STRIPE} strokeWidth={1} strokeLinecap="round" />
      </g>
      {/* Tail tip — darker accent at one end */}
      <rect x={sx + sw - 3} y={sy} width={3} height={sh} rx={1} fill={EBI_TAIL} opacity={0.5} />
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
