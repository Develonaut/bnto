"use client";

import { useCallback, useMemo, useState } from "react";
import {
  NODE_SCHEMA_DEFS,
  NODE_TYPE_INFO,
  getNodeSchema,
  getVisibleParams,
  extractSchemaDefaults,
} from "@bnto/core";
import type { NodeTypeName, NodeSchemaDefinition } from "@bnto/core";
import { SchemaForm } from "@bnto/form";
import { Badge, Card, Heading, Stack, Text } from "@bnto/ui";

/** Node types that have schema definitions, sorted alphabetically by label. */
function useSchemaNodeTypes() {
  return useMemo(() => {
    const entries = Object.entries(NODE_SCHEMA_DEFS) as [NodeTypeName, NodeSchemaDefinition][];
    return entries
      .map(([name]) => ({ name, info: NODE_TYPE_INFO[name] }))
      .sort((a, b) => a.info.label.localeCompare(b.info.label));
  }, []);
}

function SchemaFormPlayground() {
  const nodeTypes = useSchemaNodeTypes();
  const [selectedType, setSelectedType] = useState<NodeTypeName | null>(nodeTypes[0]?.name ?? null);
  const [values, setValues] = useState<Record<string, unknown>>({});

  // When selection changes, seed defaults
  const handleSelectType = useCallback((type: NodeTypeName) => {
    setSelectedType(type);
    const schema = getNodeSchema(type);
    if (schema) {
      setValues(extractSchemaDefaults(schema));
    } else {
      setValues({});
    }
  }, []);

  // Initialize defaults for the first selection on mount
  const schema = selectedType ? getNodeSchema(selectedType) : null;
  const visibleParams = useMemo(
    () => (selectedType ? getVisibleParams(selectedType, values) : []),
    [selectedType, values],
  );

  const handleChange = useCallback((name: string, value: unknown) => {
    setValues((prev) => ({ ...prev, [name]: value }));
  }, []);

  // Compute which values differ from defaults for the inspector highlight
  const defaults = useMemo(() => (schema ? extractSchemaDefaults(schema) : {}), [schema]);

  return (
    <div className="grid grid-cols-[200px_1fr_240px] gap-6">
      {/* Left: Node type selector */}
      <div className="flex flex-col gap-1">
        <Text size="xs" color="muted" className="mb-2 font-semibold uppercase tracking-wider">
          Node Types
        </Text>
        <div className="flex max-h-[600px] flex-col gap-0.5 overflow-y-auto">
          {nodeTypes.map(({ name, info }) => (
            <button
              key={name}
              type="button"
              onClick={() => handleSelectType(name)}
              className={`flex items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-sm transition-colors ${
                selectedType === name
                  ? "bg-primary/10 text-primary font-medium"
                  : "text-foreground hover:bg-muted"
              }`}
            >
              <span className="min-w-0 flex-1 truncate">{info.label}</span>
              <Badge variant="secondary" className="shrink-0 text-[10px]">
                {info.category}
              </Badge>
            </button>
          ))}
        </div>
      </div>

      {/* Center: Live form */}
      <Card className="p-4">
        {schema && selectedType ? (
          <Stack gap="md">
            <div>
              <Heading level={3} size="xs">
                {NODE_TYPE_INFO[selectedType].label}
              </Heading>
              <Text size="xs" color="muted">
                {NODE_TYPE_INFO[selectedType].description}
              </Text>
            </div>
            <SchemaForm
              schema={schema}
              values={values}
              visibleParams={visibleParams}
              onChange={handleChange}
            />
          </Stack>
        ) : (
          <Text size="sm" color="muted" className="text-center">
            Select a node type to preview its form.
          </Text>
        )}
      </Card>

      {/* Right: State inspector */}
      <div className="flex flex-col gap-2">
        <Text size="xs" color="muted" className="font-semibold uppercase tracking-wider">
          Form State
        </Text>
        <Card className="flex-1 overflow-auto p-3">
          <pre className="font-mono text-xs leading-relaxed">
            {Object.entries(values).map(([key, val]) => {
              const isModified = JSON.stringify(val) !== JSON.stringify(defaults[key]);
              return (
                <div key={key} className={isModified ? "text-primary" : "text-muted-foreground"}>
                  <span className="text-foreground/60">{key}:</span> {JSON.stringify(val)}
                </div>
              );
            })}
            {Object.keys(values).length === 0 && (
              <span className="text-muted-foreground">No values</span>
            )}
          </pre>
        </Card>
        <Text size="xs" color="muted">
          <span className="text-primary">Highlighted</span> = changed from default
        </Text>
      </div>
    </div>
  );
}

export { SchemaFormPlayground };
