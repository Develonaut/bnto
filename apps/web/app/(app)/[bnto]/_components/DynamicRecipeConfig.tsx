"use client";

import { useCallback, useMemo } from "react";
import type { Definition, NodeTypeName } from "@bnto/core";
import { getNodeSchema, getNodeParamFields, getVisibleParams, NODE_TYPE_INFO } from "@bnto/core";
import { SchemaForm } from "@bnto/form";
import { Divider, FadeIn, Heading, Stack } from "@bnto/ui";
import type { ProcessingNode } from "../_utils/extractProcessingNodes";
import { extractProcessingNodes } from "../_utils/extractProcessingNodes";

interface DynamicRecipeConfigProps {
  definition: Definition;
  config: Record<string, Record<string, unknown>>;
  onChange: (nodeId: string, paramName: string, value: unknown) => void;
  /** Input files for preview controls (source images for watermark preview). */
  files?: File[];
}

/**
 * DynamicRecipeConfig — schema-driven replacement for per-recipe config components.
 *
 * Reads the definition tree, finds processing nodes, and renders a SchemaForm
 * for each. Single-node recipes show no header; multi-node recipes show a
 * section heading per node.
 */
export function DynamicRecipeConfig({
  definition,
  config,
  onChange,
  files,
}: DynamicRecipeConfigProps) {
  const nodes = useMemo(() => extractProcessingNodes(definition), [definition]);

  if (nodes.length === 0) return null;

  return (
    <FadeIn>
      <Stack gap="lg">
        {nodes.map((node, i) => (
          <div key={node.id}>
            {i > 0 && <Divider className="mb-4" />}
            <NodeSection
              node={node}
              values={config[node.id] ?? {}}
              showHeader={nodes.length > 1}
              onChange={onChange}
              files={files}
            />
          </div>
        ))}
      </Stack>
    </FadeIn>
  );
}

interface NodeSectionProps {
  node: ProcessingNode;
  values: Record<string, unknown>;
  showHeader: boolean;
  onChange: (nodeId: string, paramName: string, value: unknown) => void;
  files?: File[];
}

export function NodeSection({ node, values, showHeader, onChange, files }: NodeSectionProps) {
  const schema = getNodeSchema(node.type);
  const handleChange = useCallback(
    (name: string, value: unknown) => onChange(node.id, name, value),
    [onChange, node.id],
  );
  if (!schema) return null;

  return (
    <div>
      {showHeader && (
        <Heading level={4} size="xs" className="mb-2 text-muted-foreground">
          {NODE_TYPE_INFO[node.type as NodeTypeName]?.label ?? node.name}
        </Heading>
      )}
      <SchemaForm
        schema={schema}
        fields={getNodeParamFields(node.type)}
        values={values}
        visibleParams={getVisibleParams(node.type, values)}
        onChange={handleChange}
        files={files}
      />
    </div>
  );
}
