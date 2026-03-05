"use client";

import { useCallback } from "react";
import {
  cn,
  Badge,
  Heading,
  Panel,
  PanelHeader,
  PanelDivider,
  PanelContent,
  Text,
  usePrevious,
} from "@bnto/ui";
import { useEditorStore } from "../../hooks/useEditorStore";
import { useEditorNode } from "../../hooks/useEditorNode";
import { useEditorActions } from "../../hooks/useEditorActions";
import { useEditorPanels } from "../../hooks/useEditorPanels";
import { SchemaForm } from "../SchemaForm";

/**
 * ConfigPanel — self-contained right-side panel.
 *
 * Reads selectedNodeId and visibility from the store.
 * Uses usePrevious so the panel keeps showing the last selected
 * node's config while it slides out (no flash to empty).
 * Includes its own slide-in overlay positioning.
 *
 * Parameter fields are rendered by SchemaForm — fully schema-driven,
 * no manual field iteration or type switching.
 */

function ConfigPanelRoot() {
  const selectedNodeId = useEditorStore((s) => s.selectedNodeId);
  const prevSelectedNodeId = usePrevious(selectedNodeId);
  const configNodeId = selectedNodeId ?? prevSelectedNodeId ?? null;

  const { configOpen } = useEditorPanels();
  const { node, config, typeInfo, schemaDef, visibleParams } = useEditorNode(configNodeId);
  const { updateParams } = useEditorActions();

  /* CSS transition for panel open/close — Animate.* covers entrance animations,
     not boolean state transitions. See animation.md decision tree. */
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
          <PanelContent>
            <div className="p-4">
              <Text size="sm" color="muted" className="text-center">
                Select a node to configure
              </Text>
            </div>
          </PanelContent>
        </Panel>
      </div>
    );
  }

  return (
    <div onPointerDownCapture={(e) => e.stopPropagation()} className={slotCn}>
      <Panel className="h-full w-full">
        <PanelHeader className="gap-2 px-3 pt-3 pb-2">
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
        </PanelHeader>
        {typeInfo.description && (
          <Text size="xs" color="muted" className="px-3 pb-1">
            {typeInfo.description}
          </Text>
        )}
        <PanelDivider />
        <PanelContent>
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
        </PanelContent>
      </Panel>
    </div>
  );
}

export { ConfigPanelRoot };
