"use client";

import { useCallback } from "react";
import { Badge, Divider, Heading, SlidersHorizontalIcon, Text, usePrevious } from "@bnto/ui";
import { isIoNodeType } from "@bnto/core";
import { useEditor } from "../../context";
import { useEditorNode } from "../../hooks/useEditorNode";
import { SchemaForm } from "@bnto/form";
import { EditorMenuPanel } from "../EditorMenuPanel";
import { ConfigPanelDeleteButton } from "./ConfigPanelDeleteButton";

/**
 * ConfigPanel — Menu-based config panel.
 *
 * Opens to the left from the right toolbar trigger. Store controls
 * open/close, Radix handles positioning. Parameter fields are
 * rendered by SchemaForm — fully schema-driven.
 */

function ConfigPanelRoot() {
  const editor = useEditor();
  const { selectedNodeId } = editor.nodes.useNodes();
  const prevSelectedNodeId = usePrevious(selectedNodeId);
  const configNodeId = selectedNodeId ?? prevSelectedNodeId ?? null;

  const { node, config, typeInfo, schemaDef, fieldConfigs, visibleParams } =
    useEditorNode(configNodeId);

  const handleParamChange = useCallback(
    (paramName: string, value: unknown) => {
      if (!configNodeId) return;
      editor.definition.updateParams(configNodeId, { [paramName]: value });
    },
    [configNodeId, editor],
  );

  const hasContent = configNodeId && node && config && typeInfo;

  return (
    <EditorMenuPanel
      panelId="config"
      side="left"
      width="w-72"
      boundaryPadding={16}
      label="Properties"
      icon={<SlidersHorizontalIcon className="size-4" />}
    >
      {!hasContent ? (
        <div className="p-4">
          <Text size="sm" color="muted" className="text-center">
            Select a node to configure
          </Text>
        </div>
      ) : (
        <>
          <div className="shrink-0 px-3 pt-3 pb-2">
            <div className="flex items-center gap-2">
              <Heading level={3} size="xs" className="min-w-0 flex-1 truncate">
                {config.displayName || config.name || typeInfo.label}
              </Heading>
              {!isIoNodeType(config.nodeType) && (
                <ConfigPanelDeleteButton
                  nodeId={configNodeId}
                  nodeName={config.displayName || config.name || typeInfo.label}
                />
              )}
            </div>
            {typeInfo.description && (
              <Text size="xs" color="muted" className="mt-1">
                {typeInfo.description}
              </Text>
            )}
            <div className="mt-2 flex items-center gap-1.5">
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
          <Divider />
          <div className="flex-1 overflow-y-auto">
            <div className="p-3">
              {schemaDef ? (
                <SchemaForm
                  schema={schemaDef}
                  fields={fieldConfigs}
                  values={config.parameters}
                  visibleParams={visibleParams}
                  onChange={handleParamChange}
                />
              ) : (
                <Text size="xs" color="muted">
                  No configurable parameters.
                </Text>
              )}
            </div>
          </div>
        </>
      )}
    </EditorMenuPanel>
  );
}

export { ConfigPanelRoot };
