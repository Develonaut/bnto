"use client";

import { Card, CardContent, Text } from "@bnto/ui";
import { SchemaForm } from "@bnto/form";
import type { ProcessingNode } from "../../_utils/extractProcessingNodes";
import { useNodeConfig } from "../../_hooks/useNodeConfig";

interface NodeConfigCardProps {
  nodes: ProcessingNode[];
  parameters: Record<string, unknown>;
  onParameterChange: (name: string, value: unknown) => void;
}

/** Schema-driven config for the first processing node's parameters. */
export function NodeConfigCard({ nodes, parameters, onParameterChange }: NodeConfigCardProps) {
  const { firstNode, schema, fields, visibleParams } = useNodeConfig(nodes, parameters);

  if (!firstNode || !schema) {
    return (
      <Card elevation="sm" className="flex items-center justify-center p-5">
        <Text size="sm" color="muted">
          No configurable parameters
        </Text>
      </Card>
    );
  }

  return (
    <Card elevation="sm" className="p-5">
      <CardContent className="p-0">
        <Text size="xs" color="muted" className="mb-3 font-medium uppercase tracking-wider">
          Settings
        </Text>
        <SchemaForm
          schema={schema}
          fields={fields}
          values={parameters}
          visibleParams={visibleParams}
          onChange={onParameterChange}
        />
      </CardContent>
    </Card>
  );
}
