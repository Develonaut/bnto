"use client";

import { useCallback } from "react";
import type { ParameterSchema } from "@bnto/nodes";
import { Badge } from "@/components/ui/Badge";
import { Card } from "@/components/ui/Card";
import { Heading } from "@/components/ui/Heading";
import { Input } from "@/components/ui/Input";
import { Label } from "@/components/ui/Label";
import { Select } from "@/components/ui/Select";
import { Stack } from "@/components/ui/Stack";
import { Switch } from "@/components/ui/Switch";
import { Text } from "@/components/ui/Text";
import { useEditorNode } from "@/editor/hooks/useEditorNode";
import { useEditorActions } from "@/editor/hooks/useEditorActions";

/**
 * NodeConfigPanel — side panel for the selected node.
 *
 * Auto-generates form fields from ParameterSchema (Atomiton pattern):
 *   string → text input
 *   number → number input with min/max
 *   boolean → switch toggle
 *   enum → select dropdown
 *
 * visibleWhen and requiredWhen handled reactively — the hook resolves
 * which params are visible given current parameter values.
 */

interface NodeConfigPanelProps {
  /** ID of the currently selected node, or null. */
  selectedNodeId: string | null;
}

function NodeConfigPanel({ selectedNodeId }: NodeConfigPanelProps) {
  const { node, typeInfo, visibleParams } = useEditorNode(selectedNodeId);
  const { updateParams } = useEditorActions();

  const handleParamChange = useCallback(
    (paramName: string, value: unknown) => {
      if (!selectedNodeId) return;
      updateParams(selectedNodeId, { [paramName]: value });
    },
    [selectedNodeId, updateParams],
  );

  if (!selectedNodeId || !node || !typeInfo) {
    return (
      <Card elevation="md" className="h-full">
        <div className="p-4">
          <Text size="sm" color="muted" className="text-center">
            Select a node to configure
          </Text>
        </div>
      </Card>
    );
  }

  return (
    <Card elevation="md" className="h-full">
      <div className="h-full overflow-y-auto p-4">
        <Stack gap="md">
          {/* Header */}
          <div>
            <Heading level={3} size="xs">
              {typeInfo.label}
            </Heading>
            <Text size="xs" color="muted" className="mt-0.5">
              {typeInfo.description}
            </Text>
            <div className="mt-2 flex gap-1.5">
              <Badge variant="secondary" className="text-xs">
                {typeInfo.category}
              </Badge>
              {typeInfo.browserCapable ? (
                <Badge variant="secondary" className="text-xs">
                  Browser
                </Badge>
              ) : (
                <Badge variant="outline" className="text-xs">
                  Pro
                </Badge>
              )}
            </div>
          </div>

          {/* Parameter fields */}
          {visibleParams.length === 0 ? (
            <Text size="xs" color="muted">
              No configurable parameters.
            </Text>
          ) : (
            <Stack gap="sm">
              {visibleParams.map((param) => (
                <ParameterField
                  key={param.name}
                  param={param}
                  value={node.parameters[param.name]}
                  onChange={handleParamChange}
                />
              ))}
            </Stack>
          )}
        </Stack>
      </div>
    </Card>
  );
}

/* ── Auto-generated field from ParameterSchema ────────────────── */

interface ParameterFieldProps {
  param: ParameterSchema;
  value: unknown;
  onChange: (name: string, value: unknown) => void;
}

function ParameterField({ param, value, onChange }: ParameterFieldProps) {
  const handleChange = useCallback(
    (newValue: unknown) => onChange(param.name, newValue),
    [param.name, onChange],
  );

  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={`param-${param.name}`} className="text-xs font-medium">
        {param.label}
        {param.required && (
          <span className="ml-0.5 text-destructive">*</span>
        )}
      </Label>

      {param.type === "enum" && param.enumValues ? (
        <Select
          value={String(value ?? param.default ?? "")}
          onValueChange={handleChange}
        >
          <Select.Trigger size="sm" id={`param-${param.name}`}>
            <Select.Value placeholder={param.placeholder ?? "Select…"} />
          </Select.Trigger>
          <Select.Content>
            {param.enumValues.map((enumVal) => (
              <Select.Item key={enumVal} value={enumVal}>
                {enumVal}
              </Select.Item>
            ))}
          </Select.Content>
        </Select>
      ) : param.type === "boolean" ? (
        <Switch
          id={`param-${param.name}`}
          checked={Boolean(value ?? param.default ?? false)}
          onCheckedChange={handleChange}
        />
      ) : param.type === "number" ? (
        <Input
          id={`param-${param.name}`}
          type="number"
          value={String(value ?? param.default ?? "")}
          min={param.min}
          max={param.max}
          placeholder={param.placeholder}
          onChange={(e) => {
            const num = e.target.value === "" ? undefined : Number(e.target.value);
            handleChange(num);
          }}
          className="h-8 text-sm"
        />
      ) : (
        /* string or fallback */
        <Input
          id={`param-${param.name}`}
          type="text"
          value={String(value ?? param.default ?? "")}
          placeholder={param.placeholder}
          onChange={(e) => handleChange(e.target.value)}
          className="h-8 text-sm"
        />
      )}

      {param.description && (
        <Text size="xs" color="muted">
          {param.description}
        </Text>
      )}
    </div>
  );
}

export { NodeConfigPanel };
