/**
 * Conveyor belt components — shared between the Motorway showcase
 * and the production visual editor.
 *
 * StationNode renders station "buildings" on the conveyor map.
 * ConveyorEdge renders animated belt connections between stations.
 * ConveyorCanvas wraps everything in a ReactFlow canvas.
 * BeltPiece / PieceShape render sushi pieces traveling on belts.
 */

export { ConveyorCanvas } from "./ConveyorCanvas";
export { StationNode, type StationData, type StationNodeType } from "./StationNode";
export {
  ConveyorEdge,
  type ConveyorEdgeData,
  type ConveyorEdgeType,
} from "./ConveyorEdge";
export {
  BeltPiece,
  PieceShape,
  VARIANT_PIECE_MAP,
  SALMON_CLIP,
  type PieceType,
} from "./BeltPiece";
