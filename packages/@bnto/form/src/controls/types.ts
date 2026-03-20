import type { NodeParamField, NodeParamFieldInfo, NodeParam } from "@bnto/core";

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
  fieldInfo: NodeParamFieldInfo;
  meta: NodeParam;
  fieldConfig?: NodeParamField;
  value: unknown;
  onChange: (value: unknown) => void;
}
