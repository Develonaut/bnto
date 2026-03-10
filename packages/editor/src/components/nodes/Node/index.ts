/**
 * Node — compound composition primitives for bento grid nodes.
 *
 * NodeRoot is the slot/card shell. NodeHeader, NodeBody, NodeFooter
 * are interior layout zones. NodeEdge positions content outside the card.
 * NodeIcon, NodeLabel are content. NodeDeleteButton is an action.
 */

export { NodeRoot, type NodeRootProps } from "./NodeRoot";
export { NodeHeader } from "./NodeHeader";
export { NodeBody } from "./NodeBody";
export { NodeFooter } from "./NodeFooter";
export { NodeEdge, type NodeEdgeProps } from "./NodeEdge";
export { NodeIcon } from "./NodeIcon";
export { NodeLabel } from "./NodeLabel";
export { NodeDeleteButton } from "./NodeDeleteButton";
