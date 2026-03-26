/**
 * Canvas node type registry — module-level constant.
 *
 * Stable reference prevents RF from unmounting/remounting all nodes on every render.
 */

import {
  CompartmentNode,
  IoNode,
  PlaceholderNode,
  ContainerGroupNode,
  AddDividerNode,
} from "../nodes";

export const CANVAS_NODE_TYPES = {
  compartment: CompartmentNode,
  io: IoNode,
  placeholder: PlaceholderNode,
  containerGroup: ContainerGroupNode,
  addDivider: AddDividerNode,
} as const;
