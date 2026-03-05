/**
 * Field control registry — maps FieldControl type to React component.
 *
 * This is the single lookup table that drives SchemaField rendering.
 * Adding a new control type = add a component file + add an entry here.
 */

import type { ComponentType } from "react";
import type { FieldControl } from "@bnto/nodes";
import type { ControlProps } from "./types";
import { SelectControl } from "./SelectControl";
import { SwitchControl } from "./SwitchControl";
import { SliderControl } from "./SliderControl";
import { NumberControl } from "./NumberControl";
import { TextControl } from "./TextControl";

/**
 * Registry: FieldControl → React component.
 *
 * | Control  | Component      | Renders                        |
 * |----------|----------------|--------------------------------|
 * | select   | SelectControl  | Dropdown (enums)               |
 * | switch   | SwitchControl  | Toggle (booleans)              |
 * | slider   | SliderControl  | Range slider (bounded numbers) |
 * | number   | NumberControl  | Number input (unbounded)       |
 * | text     | TextControl    | Text input (strings, fallback) |
 */
export const CONTROL_REGISTRY: Record<FieldControl, ComponentType<ControlProps>> = {
  select: SelectControl,
  switch: SwitchControl,
  slider: SliderControl,
  number: NumberControl,
  text: TextControl,
};

export type { ControlProps } from "./types";
