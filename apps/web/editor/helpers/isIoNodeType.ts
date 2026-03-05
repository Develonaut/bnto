/** Returns true if the node type is an I/O declaration node (input or output). */
export function isIoNodeType(nodeType: string): boolean {
  return nodeType === "input" || nodeType === "output";
}
