/**
 * Hook for accessing a single node's visual data, config, schema,
 * and visible params.
 *
 * Delegates to the `editor.nodes.useNode()` domain hook factory.
 * Kept as a thin re-export for backward compatibility with existing
 * consumers that import from `useEditorNode` directly.
 *
 * Must be inside an EditorProvider.
 */

"use client";

import { useEditor } from "../context";
import type { NodeHookResult } from "./factories/createUseNode";

function useEditorNode(nodeId: string | null): NodeHookResult {
  const editor = useEditor();
  return editor.nodes.useNode(nodeId);
}

export { useEditorNode };
export type { NodeHookResult as EditorNodeResult };
