import { Handle, Position } from "@xyflow/react";

/**
 * NodeHandles — invisible source + target handles for pipeline edge anchoring.
 *
 * Handles are visually hidden and non-interactive. They exist solely
 * to give React Flow anchor points for edge routing.
 */

const HANDLE_STYLE = { visibility: "hidden", pointerEvents: "none" } as const;

interface NodeHandlesProps {
  /** Render only source handle (e.g. Input node). */
  sourceOnly?: boolean;
  /** Render only target handle (e.g. Output node). */
  targetOnly?: boolean;
}

function NodeHandles({ sourceOnly, targetOnly }: NodeHandlesProps) {
  return (
    <>
      {!sourceOnly && <Handle type="target" position={Position.Left} style={HANDLE_STYLE} />}
      {!targetOnly && <Handle type="source" position={Position.Right} style={HANDLE_STYLE} />}
    </>
  );
}

export { NodeHandles };
