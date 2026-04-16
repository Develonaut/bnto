"use client";

import { useState, useMemo, useCallback } from "react";
import {
  getNodeSchema,
  getNodeParamFields,
  getVisibleParams,
  getParamFieldInfo,
  NODE_TYPE_INFO,
  NODE_SCHEMAS,
  type NodeSchema,
  type NodeParamFields,
} from "@bnto/core";
import { SchemaForm } from "@bnto/form";
import { Heading, Stack, Text } from "@bnto/ui";

// ─── Helpers ──────────────────────────────────────────────────────────

type NodeCategory = (typeof NODE_TYPE_INFO)[keyof typeof NODE_TYPE_INFO]["category"];

/** Group node type names by category from NODE_TYPE_INFO. */
function groupByCategory() {
  const groups = new Map<NodeCategory, Array<{ name: string; label: string }>>();
  for (const info of Object.values(NODE_TYPE_INFO)) {
    if (!NODE_SCHEMAS[info.name as keyof typeof NODE_SCHEMAS]) continue;
    const list = groups.get(info.category) ?? [];
    list.push({ name: info.name, label: info.label });
    groups.set(info.category, list);
  }
  return groups;
}

/** Extract Zod schema defaults by parsing an empty object. */
function getDefaults(schema: NodeSchema): Record<string, unknown> {
  try {
    return schema.schema.parse({}) as Record<string, unknown>;
  } catch {
    // If parse fails (missing required fields), build defaults manually
    const defaults: Record<string, unknown> = {};
    const shape = schema.schema.shape;
    for (const [key, zodType] of Object.entries(shape)) {
      const def = (zodType as { _def: { typeName: string; defaultValue?: () => unknown } })._def;
      if (def.typeName === "ZodDefault" && def.defaultValue) {
        defaults[key] = def.defaultValue();
      }
    }
    return defaults;
  }
}

const CATEGORY_ORDER: NodeCategory[] = [
  "io",
  "data",
  "file",
  "image",
  "spreadsheet",
  "control",
  "network",
  "system",
];

// ─── Component ────────────────────────────────────────────────────────

export function SchemaFormPlayground() {
  const grouped = useMemo(() => groupByCategory(), []);
  const [selected, setSelected] = useState("image");

  const schema = getNodeSchema(selected);
  const fields = getNodeParamFields(selected);

  const [values, setValues] = useState<Record<string, unknown>>(() =>
    schema ? getDefaults(schema) : {},
  );

  const handleSelect = useCallback((name: string) => {
    setSelected(name);
    const next = getNodeSchema(name);
    setValues(next ? getDefaults(next) : {});
  }, []);

  const handleChange = useCallback((name: string, value: unknown) => {
    setValues((prev) => ({ ...prev, [name]: value }));
  }, []);

  const visibleParams = useMemo(
    () => (schema ? getVisibleParams(selected, values) : []),
    [schema, selected, values],
  );

  return (
    <div className="grid grid-cols-[200px_1fr_320px] gap-6">
      {/* ── Left: Node type sidebar ──────────────────────── */}
      <nav className="flex flex-col gap-4 overflow-y-auto">
        {CATEGORY_ORDER.map((cat) => {
          const items = grouped.get(cat);
          if (!items?.length) return null;
          return (
            <div key={cat}>
              <Text size="xs" color="muted" className="mb-1 font-mono uppercase tracking-wider">
                {cat}
              </Text>
              <div className="flex flex-col">
                {items.map((item) => (
                  <button
                    key={item.name}
                    onClick={() => handleSelect(item.name)}
                    className={`rounded-md px-2.5 py-1.5 text-left text-sm transition-colors ${
                      selected === item.name
                        ? "bg-primary/10 text-primary font-medium"
                        : "text-muted-foreground hover:bg-muted hover:text-foreground"
                    }`}
                  >
                    {item.label}
                  </button>
                ))}
              </div>
            </div>
          );
        })}
      </nav>

      {/* ── Center: Live form ────────────────────────────── */}
      <div className="rounded-lg border bg-card p-6">
        <Heading level={3} size="xs" className="mb-4">
          {NODE_TYPE_INFO[selected as keyof typeof NODE_TYPE_INFO]?.label ?? selected}
        </Heading>

        {schema ? (
          <SchemaForm
            schema={schema}
            fields={fields}
            values={values}
            visibleParams={visibleParams}
            onChange={handleChange}
          />
        ) : (
          <Text color="muted">No schema available for this node type.</Text>
        )}
      </div>

      {/* ── Right: Inspector panels ─────────────────────── */}
      <div className="flex flex-col gap-4 overflow-y-auto">
        <InspectorSection
          title="Form State"
          description="Current parameter values as stored in the node config."
        >
          <pre className="overflow-x-auto text-xs">{JSON.stringify(values, null, 2)}</pre>
        </InspectorSection>

        <InspectorSection
          title="Parameters (NodeParam)"
          description="Engine-generated metadata: labels, descriptions, and visibility rules."
        >
          <SchemaInspector schema={schema} visibleParams={visibleParams} />
        </InspectorSection>

        <InspectorSection
          title="Fields (NodeParamField)"
          description="UI presentation hints: groups, presets, options, and control overrides."
        >
          <FieldsInspector fields={fields} visibleParams={visibleParams} />
        </InspectorSection>

        <InspectorSection
          title="Inferred Controls"
          description="Final control type derived from Zod schema shape and field config."
        >
          <ControlsInspector schema={schema} visibleParams={visibleParams} />
        </InspectorSection>
      </div>
    </div>
  );
}

// ─── Inspector panels ─────────────────────────────────────────────────

function InspectorSection({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="rounded-lg border bg-card p-4">
      <Text size="xs" color="muted" className="mb-2 font-mono uppercase tracking-wider">
        {title}
      </Text>
      {description && (
        <Text size="xs" color="muted" className="mb-3">
          {description}
        </Text>
      )}
      {children}
    </div>
  );
}

function SchemaInspector({
  schema,
  visibleParams,
}: {
  schema: NodeSchema | undefined;
  visibleParams: string[];
}) {
  if (!schema)
    return (
      <Text size="sm" color="muted">
        —
      </Text>
    );

  return (
    <Stack gap="sm">
      {visibleParams.map((name) => {
        const meta = schema.params[name];
        if (!meta) return null;
        return (
          <div key={name} className="text-xs">
            <Text size="xs" className="font-mono font-medium">
              {name}
            </Text>
            <div className="ml-2 text-muted-foreground">
              <div>
                {meta.label} — {meta.description}
              </div>
            </div>
          </div>
        );
      })}
    </Stack>
  );
}

function FieldsInspector({
  fields,
  visibleParams,
}: {
  fields: NodeParamFields | undefined;
  visibleParams: string[];
}) {
  if (!fields)
    return (
      <Text size="sm" color="muted">
        No field configs
      </Text>
    );

  const allKeys = Object.keys(fields);
  if (!allKeys.length)
    return (
      <Text size="sm" color="muted">
        Empty
      </Text>
    );

  return (
    <Stack gap="sm">
      {allKeys.map((name) => {
        const fc = fields[name];
        if (!fc) return null;
        const isVisible = visibleParams.includes(name);
        return (
          <div key={name} className="text-xs">
            <Text
              size="xs"
              className={`font-mono font-medium ${isVisible ? "" : "line-through opacity-50"}`}
            >
              {name}
            </Text>
            <div className="ml-2 text-muted-foreground">
              {fc.group && <span className="mr-2">group: {fc.group}</span>}
              {fc.suffix && <span className="mr-2">suffix: {fc.suffix}</span>}
              {fc.inverted && <span className="mr-2">inverted</span>}
              {fc.presets && (
                <div className="mt-0.5 font-mono text-[10px]">
                  presets: {fc.presets.map((p) => `${p.label}(${p.value})`).join(", ")}
                </div>
              )}
              {fc.options && (
                <div className="mt-0.5 font-mono text-[10px]">
                  options: {fc.options.map((o) => `${o.label}=${o.value}`).join(", ")}
                </div>
              )}
            </div>
          </div>
        );
      })}
    </Stack>
  );
}

function ControlsInspector({
  schema,
  visibleParams,
}: {
  schema: NodeSchema | undefined;
  visibleParams: string[];
}) {
  if (!schema)
    return (
      <Text size="sm" color="muted">
        —
      </Text>
    );

  return (
    <Stack gap="sm">
      {visibleParams.map((name) => {
        const info = getParamFieldInfo(schema.nodeType, name);
        if (!info) return null;
        return (
          <div key={name} className="text-xs">
            <Text size="xs" className="font-mono font-medium">
              {name}
            </Text>
            <div className="ml-2 font-mono text-muted-foreground">
              <span className="rounded bg-muted px-1">{info.control}</span>
              <span className="ml-2">{info.type}</span>
              {info.required && <span className="ml-2 text-destructive">required</span>}
              {info.min !== undefined && <span className="ml-2">min={info.min}</span>}
              {info.max !== undefined && <span className="ml-2">max={info.max}</span>}
              {info.enumValues && (
                <div className="mt-0.5 text-[10px]">[{info.enumValues.join(", ")}]</div>
              )}
            </div>
          </div>
        );
      })}
    </Stack>
  );
}
