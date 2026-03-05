"use client";

import { useCallback } from "react";
import { Badge, Heading, Panel, Stack, Text } from "@bnto/ui";
import { useEditorNode } from "@/editor/hooks/useEditorNode";
import { useEditorActions } from "@/editor/hooks/useEditorActions";
import { ParameterField } from "./ParameterField";

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

/* Panel stays always-expanded — the outer slide-in wrapper in
   EditorOverlay handles show/hide visibility. */
const noop = () => {};

interface NodeConfigPanelProps {
  /** ID of the currently selected node, or null. */
  selectedNodeId: string | null;
}

function NodeConfigPanel({ selectedNodeId }: NodeConfigPanelProps) {
  const { node, config, typeInfo, visibleParams } = useEditorNode(selectedNodeId);
  const { updateParams } = useEditorActions();

  const handleParamChange = useCallback(
    (paramName: string, value: unknown) => {
      if (!selectedNodeId) return;
      updateParams(selectedNodeId, { [paramName]: value });
    },
    [selectedNodeId, updateParams],
  );

  if (!selectedNodeId || !node || !config || !typeInfo) {
    return (
      <Panel collapsed={false} onToggle={noop} className="h-full w-full">
        <Panel.Content>
          <div className="p-4">
            <Text size="sm" color="muted" className="text-center">
              Select a node to configure
            </Text>
          </div>
        </Panel.Content>
      </Panel>
    );
  }

  return (
    <Panel collapsed={false} onToggle={noop} className="h-full w-full">
      <Panel.Header className="gap-2 px-3 pt-3 pb-2">
        <Heading level={3} size="xs" className="min-w-0 flex-1 truncate">
          {typeInfo.label}
        </Heading>
        <div className="flex gap-1.5">
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
      </Panel.Header>
      {typeInfo.description && (
        <Text size="xs" color="muted" className="px-3 pb-1">
          {typeInfo.description}
        </Text>
      )}
      <Panel.Divider />
      <Panel.Content>
        <div className="p-3">
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
                  value={config.parameters[param.name]}
                  onChange={handleParamChange}
                />
              ))}
            </Stack>
          )}
        </div>
      </Panel.Content>
    </Panel>
  );
}

export { NodeConfigPanel };
