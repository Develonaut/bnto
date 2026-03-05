import type { FieldTypeInfo, NodeParamMeta } from "@bnto/nodes";

/**
 * Props shared by all field control components.
 *
 * Each control receives the same shape — the registry map
 * dispatches to the correct component based on `fieldInfo.control`.
 */
export interface ControlProps {
  id: string;
  fieldInfo: FieldTypeInfo;
  meta: NodeParamMeta;
  value: unknown;
  onChange: (value: unknown) => void;
}
