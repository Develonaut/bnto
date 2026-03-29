"use client";

import { useMemo } from "react";
import { getNodeSchema, getNodeParamFields, getVisibleParams } from "@bnto/core";
import type { ProcessingNode } from "../_utils/extractProcessingNodes";

/** Derives schema, fields, and visible params for the first processing node. */
export function useNodeConfig(nodes: ProcessingNode[], parameters: Record<string, unknown>) {
  const firstNode = nodes[0];

  const schema = useMemo(() => (firstNode ? getNodeSchema(firstNode.type) : null), [firstNode]);

  const fields = useMemo(
    () => (firstNode ? getNodeParamFields(firstNode.type) : undefined),
    [firstNode],
  );

  const visibleParams = useMemo(
    () => (firstNode && schema ? getVisibleParams(firstNode.type, parameters) : []),
    [firstNode, schema, parameters],
  );

  return { firstNode, schema, fields, visibleParams };
}
