import type { FieldConfig, FieldTypeInfo, NodeParamMeta } from "@bnto/core";

/**
 * Props shared by all field control components.
 *
 * Each control receives the same shape — the registry map
 * dispatches to the correct component based on `fieldInfo.control`.
 *
 * - `meta` provides engine metadata (label, description, placeholder).
 * - `fieldConfig` provides UI presentation metadata (presets, options, suffix, inverted).
 */
export interface ControlProps {
  id: string;
  fieldInfo: FieldTypeInfo;
  meta: NodeParamMeta;
  fieldConfig?: FieldConfig;
  value: unknown;
  onChange: (value: unknown) => void;
}
