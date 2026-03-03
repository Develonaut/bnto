/** Half-size of each piece (radius for circles, half-width for rects).
 * Shared by all sushi piece shapes so they render at consistent scale. */
export const S = 8;

/* Nigiri topping clip dimensions — derived from S.
 * ConveyorCanvas defines a shared <clipPath id="salmon-clip"> using
 * these values so all nigiri variants can clip their topping textures. */
const NIGIRI_W = S * 1.8;
const NIGIRI_H = S * 1.5;

export const SALMON_CLIP = {
  x: -NIGIRI_W * 0.38,
  y: -NIGIRI_H * 0.33,
  width: NIGIRI_W * 0.76,
  height: NIGIRI_H * 0.66,
  rx: 2.5,
} as const;
