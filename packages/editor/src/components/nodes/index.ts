/**
 * Node components — composition primitives and RF node types.
 *
 * Node/ contains the compound primitives (NodeRoot, NodeHeader, etc.).
 * CompartmentNode, IoNode, and PlaceholderNode are the RF node types
 * that compose those primitives.
 */

// Composition primitives
export {
  NodeRoot,
  type NodeRootProps,
  NodeHeader,
  NodeBody,
  NodeFooter,
  NodeIcon,
  NodeLabel,
  NodeSublabel,
  NodeDeleteButton,
} from "./Node";

// RF node types
export { CompartmentNode, type CompartmentStatus } from "./CompartmentNode";
export { IoNode } from "./IoNode";
export { PlaceholderNode } from "./PlaceholderNode";
