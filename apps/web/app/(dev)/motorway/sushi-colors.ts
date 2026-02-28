/**
 * Sushi piece color palette — shared between BeltPiece (belt-scale)
 * and ConveyorShowcase preview (enlarged preview-scale).
 *
 * Fixed food colors so sushi looks like sushi regardless of which
 * colored belt it rides on. Pieces "pop" against the belt surface.
 */

/* ── Structural colors (nori wrap, rice) ────────────────────── */
export const NORI = "#3d3d3d";
export const RICE = "#fff";
export const RICE_STROKE = "#e0d8ce";

/* ── Nigiri toppings ────────────────────────────────────────── */
export const SALMON = "#f09384";
export const SALMON_LINE = "#ffa1a2";       /* lighter marbling stripes */
export const TAMAGO = "#f0c850";            /* golden egg block */
export const TAMAGO_GRILL = "#d4a830";      /* darker grill/fold line */
export const EBI = "#f0a08a";               /* pink-orange shrimp body */
export const EBI_STRIPE = "#f5c0b0";        /* lighter segment stripes */
export const EBI_TAIL = "#e88070";          /* darker tail tip */

/* ── Maki filling colors (cross-section centers) ────────────── */
export const FILLING_TUNA = "#c0394f";      /* deep red — tekka maki */
export const FILLING_SALMON = "#e8856e";     /* orange-pink — sake maki */
export const FILLING_SALMON_LINE = "#f2a48e"; /* marbling in salmon filling */
export const FILLING_SHRIMP = "#f0b0a8";     /* light pink — ebi maki */
export const FILLING_SCALLOP = "#f5d5c0";    /* pale cream — hotate maki */
export const FILLING_EGG = "#f0c850";        /* golden yellow — tamago maki */
export const FILLING_CUCUMBER = "#7ecf7a";   /* green — kappa maki */
export const FILLING_CUCUMBER_DARK = "#52c64e"; /* darker green for texture */

/** Default filling — used by the basic "maki" type */
export const FILLING_GREEN = "#7ecf7a";
