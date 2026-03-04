"use client";

import { useStore } from "@xyflow/react";
import type { CompartmentNodeType } from "@/components/editor/canvas/CompartmentNode";

/** Subscribe to the ReactFlow node array, typed as CompartmentNodeType[]. */
export function useCanvasNodes(): CompartmentNodeType[] {
  return useStore((s) => s.nodes) as CompartmentNodeType[];
}
