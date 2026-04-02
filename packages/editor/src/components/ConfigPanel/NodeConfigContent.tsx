"use client";

import type { ReactNode } from "react";
import { Badge, Divider, Heading, Text } from "@bnto/ui";
import { isIoNodeType } from "@bnto/core";
import { SchemaForm } from "@bnto/form";
import type { useEditorNode } from "../../hooks/useEditorNode";

/**
 * NodeConfigContent — the config panel body when a node is selected.
 *
 * Renders header (name + badges) and schema-driven parameter form.
 */

interface NodeConfigContentProps {
  configNodeId: string;
  nodeData: ReturnType<typeof useEditorNode>;
  onParamChange: (paramName: string, value: unknown) => void;
  deleteButton: ReactNode;
}

function NodeConfigContent({ nodeData, onParamChange, deleteButton }: NodeConfigContentProps) {
  const { config, typeInfo, schemaDef, fieldConfigs, visibleParams } = nodeData;
  if (!config || !typeInfo) return null;

  return (
    <>
      <NodeConfigHeader config={config} typeInfo={typeInfo} deleteButton={deleteButton} />
      <Divider />
      <NodeConfigForm
        schemaDef={schemaDef}
        fieldConfigs={fieldConfigs}
        values={config.parameters}
        visibleParams={visibleParams}
        onChange={onParamChange}
      />
    </>
  );
}

/** Schema-driven parameter form section. */
function NodeConfigForm({
  schemaDef,
  fieldConfigs,
  values,
  visibleParams,
  onChange,
}: {
  schemaDef: ReturnType<typeof useEditorNode>["schemaDef"];
  fieldConfigs: ReturnType<typeof useEditorNode>["fieldConfigs"];
  values: Record<string, unknown>;
  visibleParams: ReturnType<typeof useEditorNode>["visibleParams"];
  onChange: (paramName: string, value: unknown) => void;
}) {
  return (
    <div className="flex-1 overflow-y-auto">
      <div className="p-3">
        <NodeConfigFormContent
          schemaDef={schemaDef}
          fieldConfigs={fieldConfigs}
          values={values}
          visibleParams={visibleParams}
          onChange={onChange}
        />
      </div>
    </div>
  );
}

/** Inner content — renders SchemaForm or empty message. */
function NodeConfigFormContent({
  schemaDef,
  fieldConfigs,
  values,
  visibleParams,
  onChange,
}: {
  schemaDef: ReturnType<typeof useEditorNode>["schemaDef"];
  fieldConfigs: ReturnType<typeof useEditorNode>["fieldConfigs"];
  values: Record<string, unknown>;
  visibleParams: ReturnType<typeof useEditorNode>["visibleParams"];
  onChange: (paramName: string, value: unknown) => void;
}) {
  if (!schemaDef) {
    return (
      <Text size="xs" color="muted">
        No configurable parameters.
      </Text>
    );
  }
  return (
    <SchemaForm
      schema={schemaDef}
      fields={fieldConfigs}
      values={values}
      visibleParams={visibleParams}
      onChange={onChange}
    />
  );
}

/** Platform capability badge. */
function CapabilityBadge({ platforms }: { platforms: readonly string[] }) {
  return platforms.includes("browser") ? (
    <Badge variant="secondary" className="text-xs">
      Browser
    </Badge>
  ) : (
    <Badge variant="outline" className="text-xs">
      Pro
    </Badge>
  );
}

/** Category + capability badge row. */
function NodeBadges({ category, platforms }: { category: string; platforms: readonly string[] }) {
  return (
    <div className="mt-2 flex items-center gap-1.5">
      <Badge variant="secondary" className="text-xs">
        {category}
      </Badge>
      <CapabilityBadge platforms={platforms} />
    </div>
  );
}

/** Header section: name, description, category badges. */
function NodeConfigHeader({
  config,
  typeInfo,
  deleteButton,
}: {
  config: NonNullable<ReturnType<typeof useEditorNode>["config"]>;
  typeInfo: NonNullable<ReturnType<typeof useEditorNode>["typeInfo"]>;
  deleteButton: ReactNode;
}) {
  return (
    <div className="shrink-0 px-3 pt-3 pb-2">
      <div className="flex items-center gap-2">
        <Heading level={3} size="xs" className="min-w-0 flex-1 truncate">
          {config.displayName || config.name || typeInfo.label}
        </Heading>
        {!isIoNodeType(config.nodeType) && deleteButton}
      </div>
      {typeInfo.description && (
        <Text size="xs" color="muted" className="mt-1">
          {typeInfo.description}
        </Text>
      )}
      <NodeBadges category={typeInfo.category} platforms={typeInfo.platforms} />
    </div>
  );
}

export { NodeConfigContent };
