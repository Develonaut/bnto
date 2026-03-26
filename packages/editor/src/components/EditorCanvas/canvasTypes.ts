/**
 * Canvas prop types — shared between Canvas and CanvasInner.
 */

import type { ReactNode } from "react";
import type { NodeChange, EdgeChange, Edge } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";

export interface CanvasProps {
  nodes?: BentoNode[];
  onNodesChange?: (changes: NodeChange<BentoNode>[]) => void;
  edges?: Edge[];
  onEdgesChange?: (changes: EdgeChange[]) => void;
  defaultNodes?: BentoNode[];
  height?: number;
  interactive?: boolean;
  disable?: { drag?: boolean; pan?: boolean; select?: boolean };
  standalone?: boolean;
  onNodeClick?: (nodeId: string) => void;
  onPaneClick?: () => void;
  children?: ReactNode;
  className?: string;
}
