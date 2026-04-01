/**
 * Watermark/overlay layout — matches the Rust engine's
 * `calculate_position()` in `engine/crates/bnto-image/src/overlay.rs`.
 *
 * The engine uses gravity-based positioning: overlays are placed fully
 * visible within their gravity region, with a 2% margin from nearest edges.
 * Center/middle axes use true centering with no margin.
 */

/** 2% margin matching `src_w * 0.02` in the engine's calculate_position(). */
export const ENGINE_MARGIN_PERCENT = 2;

/** The 9 position values matching the engine's position enum. */
export const POSITIONS = [
  "top-left",
  "top-center",
  "top-right",
  "middle-left",
  "center",
  "middle-right",
  "bottom-left",
  "bottom-center",
  "bottom-right",
] as const;

function getColumn(position: string): "left" | "center" | "right" {
  if (position.includes("left")) return "left";
  if (position.includes("right")) return "right";
  return "center";
}

function getRow(position: string): "top" | "middle" | "bottom" {
  if (position.startsWith("top")) return "top";
  if (position.startsWith("bottom")) return "bottom";
  return "middle";
}

/**
 * Compute CSS styles for the image area that maintains aspect ratio.
 *
 * The preview container is square. The image area is inset to match the
 * source image's aspect ratio using letterboxing (bars on shorter axis).
 */
export function imageAreaStyle(aspectRatio: number): React.CSSProperties {
  if (aspectRatio >= 1) {
    const heightPercent = (1 / aspectRatio) * 100;
    const inset = (100 - heightPercent) / 2;
    return { top: `${inset}%`, bottom: `${inset}%`, left: 0, right: 0 };
  }
  const widthPercent = aspectRatio * 100;
  const inset = (100 - widthPercent) / 2;
  return { top: 0, bottom: 0, left: `${inset}%`, right: `${inset}%` };
}

/**
 * Compute CSS position for the overlay within the image area.
 *
 * Matches the engine's gravity-based `calculate_position()`:
 * - Edge columns/rows: overlay placed fully visible with 2% margin
 * - Center/middle: overlay centered on that axis via translate(-50%)
 * - Pixel offsets applied via CSS transform
 */
export function overlayPositionStyle(
  position: string,
  sizePercent: number,
  offsetX = 0,
  offsetY = 0,
): React.CSSProperties {
  const col = getColumn(position);
  const row = getRow(position);
  const m = ENGINE_MARGIN_PERCENT;

  const style: React.CSSProperties = { position: "absolute", width: `${sizePercent}%` };

  if (col === "left") style.left = `${m}%`;
  else if (col === "right") style.right = `${m}%`;
  else style.left = "50%";

  if (row === "top") style.top = `${m}%`;
  else if (row === "bottom") style.bottom = `${m}%`;
  else style.top = "50%";

  const tx = col === "center" ? `calc(-50% + ${offsetX}px)` : `${offsetX}px`;
  const ty = row === "middle" ? `calc(-50% + ${offsetY}px)` : `${offsetY}px`;
  style.transform = `translate(${tx}, ${ty})`;

  return style;
}
