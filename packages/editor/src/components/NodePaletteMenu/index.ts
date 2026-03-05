/**
 * NodePaletteMenu — composable Menu for adding nodes to the canvas.
 *
 * Compound component assembled via Object.assign for dot-notation usage.
 */

import { NodePaletteMenuRoot, NodePaletteMenuTrigger } from "./NodePaletteMenuRoot";
import { NodePaletteMenuContent } from "./NodePaletteMenuContent";

export const NodePaletteMenu = Object.assign(NodePaletteMenuRoot, {
  Root: NodePaletteMenuRoot,
  Trigger: NodePaletteMenuTrigger,
  Content: NodePaletteMenuContent,
});
