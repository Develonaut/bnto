"use client";

import { useCallback } from "react";
import { cn } from "@/lib/cn";
import { Badge } from "@/components/ui/Badge";
import { Heading } from "@/components/ui/Heading";
import { Panel } from "@/components/ui/Panel";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { usePrevious } from "@/components/ui/usePrevious";
import { useEditorStore } from "@/editor/hooks/useEditorStore";
import { useEditorNode } from "@/editor/hooks/useEditorNode";
import { useEditorActions } from "@/editor/hooks/useEditorActions";
import { useEditorPanels } from "@/editor/hooks/useEditorPanels";
import { ParameterField } from "../ParameterField";

/**
 * ConfigPanel — self-contained right-side panel.
 *
 * Reads selectedNodeId and visibility from the store.
 * Uses usePrevious so the panel keeps showing the last selected
 * node's config while it slides out (no flash to empty).
 * Includes its own slide-in overlay positioning.
 */

function ConfigPanelRoot() {
  const selectedNodeId = useEditorStore((s) => s.selectedNodeId);
  const prevSelectedNodeId = usePrevious(selectedNodeId);
  const configNodeId = selectedNodeId ?? prevSelectedNodeId ?? null;

  const { configOpen } = useEditorPanels();
  const { node, config, typeInfo, visibleParams } =
    useEditorNode(configNodeId);
  const { updateParams } = useEditorActions();

  const slotCn = cn(
    "pointer-events-auto absolute bottom-0 right-0 top-0 w-72",
    "motion-safe:transition-[translate,opacity]",
    configOpen
      ? "translate-x-0 opacity-100 motion-safe:duration-slow motion-safe:ease-spring-bouncy"
      : "pointer-events-none translate-x-[110%] opacity-0 motion-safe:duration-fast motion-safe:ease-out",
  );

  const handleParamChange = useCallback(
    (paramName: string, value: unknown) => {
      if (!configNodeId) return;
      updateParams(configNodeId, { [paramName]: value });
    },
    [configNodeId, updateParams],
  );

  if (!configNodeId || !node || !config || !typeInfo) {
    return (
      <div onPointerDownCapture={(e) => e.stopPropagation()} className={slotCn}>
        <Panel className="h-full w-full">
          <Panel.Content>
            <div className="p-4">
              <Text size="sm" color="muted" className="text-center">
                Select a node to configure
              </Text>
            </div>
          </Panel.Content>
        </Panel>
      </div>
    );
  }

  return (
    <div onPointerDownCapture={(e) => e.stopPropagation()} className={slotCn}>
    <Panel className="h-full w-full">
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
    </div>
  );
}

export { ConfigPanelRoot };
