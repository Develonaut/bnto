import { useMemo, useCallback } from "react";
import type { NodeChange } from "@xyflow/react";
import { injectPlaceholder } from "../../../helpers/archive/rf/injectPlaceholder";
import { filterPlaceholderChanges } from "../../../helpers/archive/rf/filterPlaceholderChanges";
import type { BentoNode } from "../../../adapters/types";

/**
 * Wraps the editor canvas nodes to inject a placeholder before the
 * output node when the recipe is empty (no compartment nodes).
 *
 * When compartment nodes exist the placeholder is skipped — add-node
 * buttons on each compartment node handle adding.
 *
 * Also filters RF changes targeting the placeholder.
 */
function usePlaceholderNodes(
  nodes: BentoNode[],
  onNodesChange: (changes: NodeChange<BentoNode>[]) => void,
) {
  const displayNodes = useMemo(() => {
    const hasCompartmentNodes = nodes.some((n) => n.type === "compartment");
    return hasCompartmentNodes ? nodes : injectPlaceholder(nodes);
  }, [nodes]);

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
