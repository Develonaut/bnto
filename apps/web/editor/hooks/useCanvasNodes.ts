"use client";

import { useEditorStore } from "./useEditorStore";
import type { BentoNode } from "../adapters/types";

/** Subscribe to the editor store's node array. */
export function useCanvasNodes(): BentoNode[] {
  return useEditorStore((s) => s.nodes);
}
