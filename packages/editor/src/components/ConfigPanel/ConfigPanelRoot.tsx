"use client";

import { useCallback } from "react";
import { Badge, Divider, Heading, SlidersHorizontalIcon, Text, usePrevious } from "@bnto/ui";
import { useEditorStore } from "../../hooks/useEditorStore";
import { useEditorStoreApi } from "../../hooks/useEditorStoreApi";
import { useEditorNode } from "../../hooks/useEditorNode";
import { useEditorActions } from "../../hooks/useEditorActions";
import { SchemaForm } from "../SchemaForm";
import { SurfacedParamsSection } from "./SurfacedParamsSection";
import { updateSurfacedParam } from "../../actions/updateSurfacedParam";
import { EditorMenuPanel } from "../EditorMenuPanel";

/**
 * ConfigPanel — Menu-based config panel.
 *
 * Opens to the left from the right toolbar trigger. Store controls
 * open/close, Radix handles positioning. Parameter fields are
 * rendered by SchemaForm — fully schema-driven.
 */

function ConfigPanelRoot() {
  const selectedNodeId = useEditorStore((s) => s.selectedNodeId);
  const prevSelectedNodeId = usePrevious(selectedNodeId);
  const configNodeId = selectedNodeId ?? prevSelectedNodeId ?? null;

  const { node, config, typeInfo, schemaDef, visibleParams, surfacedGroups } =
    useEditorNode(configNodeId);
  const { updateParams } = useEditorActions();
  const storeApi = useEditorStoreApi();

  const handleParamChange = useCallback(
    (paramName: string, value: unknown) => {
      if (!configNodeId) return;
      updateParams(configNodeId, { [paramName]: value });
    },
    [configNodeId, updateParams],
  );

  const handleSurfacedParamChange = useCallback(
    (leafNodeId: string, paramName: string, value: unknown) => {
      const state = storeApi.getState();
      const result = updateSurfacedParam(state, leafNodeId, { [paramName]: value });
      if (result) storeApi.setState(result);
    },
    [storeApi],
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
          <div className="flex shrink-0 items-center gap-2 px-3 pt-3 pb-2">
            <Heading level={3} size="xs" className="min-w-0 flex-1 truncate">
              {config.displayName || config.name || typeInfo.label}
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
          </div>
          {typeInfo.description && (
            <Text size="xs" color="muted" className="px-3 pb-1">
              {typeInfo.description}
            </Text>
          )}
          <Divider />
          <div className="flex-1 overflow-y-auto">
            {surfacedGroups.length > 0 ? (
              <>
                <SurfacedParamsSection
                  groups={surfacedGroups}
                  onParamChange={handleSurfacedParamChange}
                />
                {schemaDef && visibleParams.length > 0 && (
                  <>
                    <Divider />
                    <div className="px-3 pt-1">
                      <Text
                        size="xs"
                        color="muted"
                        className="font-medium uppercase tracking-wider"
                      >
                        Advanced
                      </Text>
                    </div>
                    <div className="p-3">
                      <SchemaForm
                        schema={schemaDef}
                        values={config.parameters}
                        visibleParams={visibleParams}
                        onChange={handleParamChange}
                      />
                    </div>
                  </>
                )}
              </>
            ) : (
              <div className="p-3">
                {schemaDef ? (
                  <SchemaForm
                    schema={schemaDef}
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
            )}
          </div>
        </>
      )}
    </EditorMenuPanel>
  );
}

export { ConfigPanelRoot };
