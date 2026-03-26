"use client";

import { useCallback } from "react";
import { SlidersHorizontalIcon, usePrevious } from "@bnto/ui";
import { useEditor } from "../../context";
import { useEditorNode } from "../../hooks/useEditorNode";
import { EditorMenuPanel } from "../EditorMenuPanel";
import { ConfigPanelDeleteButton } from "./ConfigPanelDeleteButton";
import { RecipeSettingsPanel } from "./RecipeSettingsPanel";
import { NodeConfigContent } from "./NodeConfigContent";

/**
 * ConfigPanel — Menu-based config panel.
 *
 * Opens to the left from the right toolbar trigger. Store controls
 * open/close, Radix handles positioning. Parameter fields are
 * rendered by SchemaForm — fully schema-driven.
 */

/** Resolve the current config node ID (persists across deselection transitions). */
function useConfigNodeId() {
  const editor = useEditor();
  const { selectedNodeId } = editor.nodes.useNodes();
  const prev = usePrevious(selectedNodeId);
  return selectedNodeId ?? prev ?? null;
}

function ConfigPanelRoot() {
  const editor = useEditor();
  const configNodeId = useConfigNodeId();
  const nodeData = useEditorNode(configNodeId);

  const handleParamChange = useCallback(
    (paramName: string, value: unknown) => {
      if (configNodeId) editor.definition.updateParams(configNodeId, { [paramName]: value });
    },
    [configNodeId, editor],
  );

  const { config, typeInfo } = nodeData;
  const nodeName = config && typeInfo ? config.displayName || config.name || typeInfo.label : "";
  const hasContent = configNodeId && nodeData.node && config && typeInfo;

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
        <RecipeSettingsPanel />
      ) : (
        <NodeConfigContent
          configNodeId={configNodeId}
          nodeData={nodeData}
          onParamChange={handleParamChange}
          deleteButton={<ConfigPanelDeleteButton nodeId={configNodeId} nodeName={nodeName} />}
        />
      )}
    </EditorMenuPanel>
  );
}

export { ConfigPanelRoot };
