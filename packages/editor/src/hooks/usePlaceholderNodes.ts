import { useMemo, useCallback } from "react";
import type { NodeChange } from "@xyflow/react";
import { injectPlaceholder } from "../helpers/injectPlaceholder";
import { filterPlaceholderChanges } from "../helpers/filterPlaceholderChanges";
import type { BentoNode } from "../adapters/types";

/**
 * Wraps the editor canvas nodes to always inject a placeholder
 * before the output node, and filters RF changes targeting it.
 */
function usePlaceholderNodes(
  nodes: BentoNode[],
  onNodesChange: (changes: NodeChange<BentoNode>[]) => void,
) {
  const displayNodes = useMemo(() => injectPlaceholder(nodes), [nodes]);

  const handleNodesChange = useCallback(
    (changes: NodeChange<BentoNode>[]) => {
      const filtered = filterPlaceholderChanges(changes);
      if (filtered.length > 0) onNodesChange(filtered);
    },
    [onNodesChange],
  );

  return { displayNodes, handleNodesChange };
}

export { usePlaceholderNodes };
