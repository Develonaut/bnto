/**
 * Sushi pieces that travel along conveyor belt paths.
 *
 * Three shape families, each in its own file under sushi-pieces/:
 *
 *   Maki rolls — concentric circles: nori → rice → colored filling
 *   Nigiri     — rice pad → topping → nori band (salmon, tamago, ebi)
 *   Onigiri    — rounded rice body → nori wrap at base
 *
 * Uses SVG <animateMotion> to follow the belt's path data.
 * Wrapped in .belt-pieces which hides in reduced-motion mode.
 */

import { PieceShape } from "./PieceShape";

export { SALMON_CLIP } from "./sushi-pieces/constants";

/** All available sushi piece types. */
export type PieceType =
  /* Maki rolls (cross-section) — same shape, different filling */
  | "tekka"         /* tuna — deep red */
  | "sake"          /* salmon — orange-pink with marbling */
  | "ebi"           /* shrimp — light pink */
  | "hotate"        /* scallop — pale cream */
  | "tamago"        /* egg — golden yellow */
  | "kappa"         /* cucumber — green */
  /* Nigiri (top-down) — same rice pad + nori band, different topping */
  | "nigiri"        /* salmon nigiri — pink with marbling */
  | "tamago-nigiri" /* egg nigiri — golden yellow block */
  | "ebi-nigiri"    /* shrimp nigiri — pink with segment stripes */
  /* Distinct shapes */
  | "onigiri";      /* rice ball — rounded triangle */

/** Map station variant → default sushi piece type.
 * Used when pieceType isn't explicitly set on a belt — the belt
 * automatically gets the sushi type matching its source station. */
export const VARIANT_PIECE_MAP: Record<string, PieceType> = {
  muted: "tekka",
  primary: "nigiri",
  secondary: "onigiri",
  accent: "tamago",
  success: "sake",
};

type BeltPieceProps = {
  /** SVG path data string (from getSmoothStepPath) */
  path: string;
  /** Total traversal time in seconds */
  duration: number;
  /** Start delay — staggers multiple pieces on the same belt */
  begin: number;
  /** Visual type of the sushi piece */
  type?: PieceType;
};

export function BeltPiece({
  path,
  duration,
  begin,
  type = "tekka",
}: BeltPieceProps) {
  return (
    <g className="belt-piece" filter="url(#piece-shadow)">
      <PieceShape type={type} />

      {/* animateMotion moves this <g> along the belt path.
       * rotate="auto" keeps the piece oriented along the belt direction. */}
      <animateMotion
        dur={`${duration}s`}
        repeatCount="indefinite"
        path={path}
        rotate="auto"
        begin={`${begin}s`}
      />
    </g>
  );
}
