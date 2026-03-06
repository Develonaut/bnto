import { useMemo, useCallback } from "react";
import type { NodeChange } from "@xyflow/react";
import { useEditorStore } from "./useEditorStore";
import { isIoNodeType } from "../helpers/isIoNodeType";
import { injectPlaceholder } from "../helpers/injectPlaceholder";
import { filterPlaceholderChanges } from "../helpers/filterPlaceholderChanges";
import type { BentoNode } from "../adapters/types";

/**
 * Wraps the editor canvas nodes to inject a placeholder when the
 * canvas only has I/O nodes, and filters RF changes targeting it.
 */
function usePlaceholderNodes(
  nodes: BentoNode[],
  onNodesChange: (changes: NodeChange<BentoNode>[]) => void,
) {
  const onlyIoNodes = useEditorStore((s) =>
    Object.values(s.configs).every((c) => isIoNodeType(c.nodeType)),
  );

  const displayNodes = useMemo(() => injectPlaceholder(nodes, onlyIoNodes), [nodes, onlyIoNodes]);

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
