/** Must be used inside a ReactFlowProvider. */

"use client";

import { useState, useCallback } from "react";
import { useOnSelectionChange } from "@xyflow/react";
import type { Node } from "@xyflow/react";

function useEditorSelection(): { selectedNodeId: string | null } {
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);

  const onChange = useCallback(({ nodes }: { nodes: Node[] }) => {
    setSelectedNodeId(nodes.length === 1 ? nodes[0]!.id : null);
  }, []);

  useOnSelectionChange({ onChange });

  return { selectedNodeId };
}

export { useEditorSelection };
