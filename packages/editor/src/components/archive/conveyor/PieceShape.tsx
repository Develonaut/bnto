/**
 * Renders the correct sushi shape for a piece type.
 *
 * Used by BeltPiece (belt-scale animation) and ConveyorShowcase
 * (enlarged static preview). Extracted so both consumers import
 * the same component without pulling in BeltPiece's animation logic.
 */

import {
  FILLING_TUNA,
  FILLING_SALMON,
  FILLING_SALMON_LINE,
  FILLING_SHRIMP,
  FILLING_SCALLOP,
  FILLING_EGG,
  FILLING_CUCUMBER,
  FILLING_CUCUMBER_DARK,
} from "./sushi-colors";
import { MakiShape } from "./sushi-pieces/MakiShape";
import { NigiriShape } from "./sushi-pieces/NigiriShape";
import { TamagoNigiriShape } from "./sushi-pieces/TamagoNigiriShape";
import { EbiNigiriShape } from "./sushi-pieces/EbiNigiriShape";
import { OnigiriShape } from "./sushi-pieces/OnigiriShape";
import type { PieceType } from "./BeltPiece";

export function PieceShape({ type }: { type: PieceType }) {
  switch (type) {
    /* Maki rolls */
    case "tekka": return <MakiShape fill={FILLING_TUNA} />;
    case "sake": return <MakiShape fill={FILLING_SALMON} line={FILLING_SALMON_LINE} />;
    case "ebi": return <MakiShape fill={FILLING_SHRIMP} />;
    case "hotate": return <MakiShape fill={FILLING_SCALLOP} />;
    case "tamago": return <MakiShape fill={FILLING_EGG} />;
    case "kappa": return <MakiShape fill={FILLING_CUCUMBER} line={FILLING_CUCUMBER_DARK} />;
    /* Nigiri variants */
    case "nigiri": return <NigiriShape />;
    case "tamago-nigiri": return <TamagoNigiriShape />;
    case "ebi-nigiri": return <EbiNigiriShape />;
    /* Rice ball */
    case "onigiri": return <OnigiriShape />;
  }
}
