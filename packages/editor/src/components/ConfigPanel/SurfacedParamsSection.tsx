"use client";

import { useCallback } from "react";
import type { SurfacedGroup } from "@bnto/nodes";
import { Heading, PanelDivider } from "@bnto/ui";
import { SchemaForm } from "../SchemaForm";

interface SurfacedParamsSectionProps {
  groups: SurfacedGroup[];
  onParamChange: (leafNodeId: string, paramName: string, value: unknown) => void;
}

/**
 * Renders surfaced leaf node parameters inside a container's config panel.
 *
 * Single group: renders params directly (no redundant heading).
 * Multiple groups: each group gets a heading + divider separator.
 */
function SurfacedParamsSection({ groups, onParamChange }: SurfacedParamsSectionProps) {
  if (groups.length === 0) return null;

  const showHeadings = groups.length > 1;

  return (
    <>
      {groups.map((group, i) => (
        <SurfacedGroupSection
          key={group.leafNodeId}
          group={group}
          showHeading={showHeadings}
          showDivider={showHeadings && i > 0}
          onParamChange={onParamChange}
        />
      ))}
    </>
  );
}

function SurfacedGroupSection({
  group,
  showHeading,
  showDivider,
  onParamChange,
}: {
  group: SurfacedGroup;
  showHeading: boolean;
  showDivider: boolean;
  onParamChange: (leafNodeId: string, paramName: string, value: unknown) => void;
}) {
  const handleChange = useCallback(
    (paramName: string, value: unknown) => {
      onParamChange(group.leafNodeId, paramName, value);
    },
    [group.leafNodeId, onParamChange],
  );

  return (
    <div className="flex flex-col gap-2">
      {showDivider && <PanelDivider />}
      {showHeading && (
        <div className="px-3 pt-1">
          <Heading level={4} size="xs" className="text-muted-foreground">
            {group.label}
          </Heading>
        </div>
      )}
      <div className="px-3 pb-2">
        <SchemaForm
          schema={group.schema}
          values={group.values}
          visibleParams={group.visibleParams}
          onChange={handleChange}
        />
      </div>
    </div>
  );
}

export { SurfacedParamsSection };
