/**
 * @bnto/form — Schema-driven form rendering.
 *
 * Maps engine-generated Zod schemas to UI controls via a type-dispatch registry.
 * Headless with respect to state — consumers provide values and onChange.
 */

export { SchemaForm } from "./SchemaForm";
export type { SchemaFormProps } from "./SchemaForm";
export { SchemaField } from "./SchemaField";
export type { SchemaFieldProps } from "./SchemaField";
export { FieldGroup } from "./FieldGroup";
export type { FieldGroupProps, GroupField } from "./FieldGroup";
export { CONTROL_REGISTRY } from "./controls";
export type { ControlProps } from "./controls/types";
