"use client";

import { useCallback, useEffect, useState } from "react";
import {
  SlidersHorizontalIcon,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Text,
  usePrevious,
} from "@bnto/ui";
import { useEditor } from "../../context";
import { useEditorNode } from "../../hooks/useEditorNode";
import { EditorMenuPanel } from "../EditorMenuPanel";
import { ConfigPanelDeleteButton } from "./ConfigPanelDeleteButton";
import { RecipeSettingsPanel } from "./RecipeSettingsPanel";
import { NodeConfigContent } from "./NodeConfigContent";

/**
 * ConfigPanel — Menu-based config panel with Recipe / Node tabs.
 *
 * Opens to the left from the right toolbar trigger. Tabs let users
 * switch between recipe-level settings and node configuration.
 * Selecting a node auto-switches to the Node tab.
 */

type ConfigTab = "recipe" | "node";

/** Resolve the current config node ID (persists across deselection transitions). */
function useConfigNodeId() {
  const editor = useEditor();
  const { selectedNodeId } = editor.nodes.useNodes();
  const prev = usePrevious(selectedNodeId);
  return { configNodeId: selectedNodeId ?? prev ?? null, selectedNodeId };
}

/** Resolves param change handler + derived display data for the config panel. */
function useConfigPanel() {
  const editor = useEditor();
  const { configNodeId, selectedNodeId } = useConfigNodeId();
  const nodeData = useEditorNode(configNodeId);

  const handleParamChange = useCallback(
    (paramName: string, value: unknown) => {
      if (configNodeId) editor.definition.updateParams(configNodeId, { [paramName]: value });
    },
    [configNodeId, editor],
  );

  const { config, typeInfo } = nodeData;
  const nodeName = config && typeInfo ? config.displayName || config.name || typeInfo.label : "";

  return { configNodeId, selectedNodeId, nodeData, handleParamChange, nodeName };
}

function ConfigPanelRoot() {
  const { configNodeId, selectedNodeId, nodeData, handleParamChange, nodeName } = useConfigPanel();
  const { config, typeInfo, node } = nodeData;
  const hasNodeContent = configNodeId && node && config && typeInfo;

  const [activeTab, setActiveTab] = useState<ConfigTab>("recipe");
  const handleTabChange = useCallback((v: string) => setActiveTab(v as ConfigTab), []);

  useEffect(() => {
    if (selectedNodeId) setActiveTab("node");
  }, [selectedNodeId]);

  return (
    <EditorMenuPanel
      panelId="config"
      side="left"
      width="w-72"
      boundaryPadding={16}
      label="Properties"
      icon={<SlidersHorizontalIcon className="size-4" />}
    >
      <Tabs
        value={activeTab}
        onValueChange={handleTabChange}
        className="flex flex-col flex-1 min-h-0"
      >
        <div className="shrink-0 px-3 pt-3">
          <TabsList fullWidth>
            <TabsTrigger value="recipe">Recipe</TabsTrigger>
            <TabsTrigger value="node">Node</TabsTrigger>
          </TabsList>
        </div>
        <TabsContent value="recipe" className="flex-1 min-h-0 flex flex-col overflow-hidden">
          <RecipeSettingsPanel />
        </TabsContent>
        <TabsContent value="node" className="flex-1 min-h-0 flex flex-col overflow-hidden">
          {hasNodeContent ? (
            <NodeConfigContent
              configNodeId={configNodeId}
              nodeData={nodeData}
              onParamChange={handleParamChange}
              deleteButton={<ConfigPanelDeleteButton nodeId={configNodeId} nodeName={nodeName} />}
            />
          ) : (
            <div className="px-3 pt-3">
              <Text size="xs" color="muted">
                Select a node to configure
              </Text>
            </div>
          )}
        </TabsContent>
      </Tabs>
    </EditorMenuPanel>
  );
}

export { ConfigPanelRoot };
