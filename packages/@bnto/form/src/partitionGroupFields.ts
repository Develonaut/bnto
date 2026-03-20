import type { GroupField } from "./FieldGroup";
import { INLINE_CONTROLS } from "./controlCategories";

/**
 * Split grouped fields into inline (switch/select) and grid (everything else).
 *
 * Inline fields render as individual rows at the top of the group.
 * Grid fields render side-by-side in a 2-column layout below.
 */
export function partitionGroupFields(fields: GroupField[]) {
  const inlineFields = fields.filter((f) => INLINE_CONTROLS.has(f.fieldInfo.control));
  const gridFields = fields.filter((f) => !INLINE_CONTROLS.has(f.fieldInfo.control));
  return { inlineFields, gridFields };
}
