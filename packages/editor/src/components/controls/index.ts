/**
 * Field control registry — maps NodeParamControl type to React component.
 *
 * This is the single lookup table that drives SchemaField rendering.
 * Adding a new control type = add a component file + add an entry here.
 */

import type { ComponentType } from "react";
import type { NodeParamControl } from "@bnto/core";
import type { ControlProps } from "./types";
import { SelectControl } from "./SelectControl";
import { SwitchControl } from "./SwitchControl";
import { SliderControl } from "./SliderControl";
import { NumberControl } from "./NumberControl";
import { TextControl } from "./TextControl";
import { TextareaControl } from "./TextareaControl";
import { TagPickerControl } from "./TagPickerControl";
import { KeyValueEditorControl } from "./KeyValueEditorControl";

/**
 * Registry: NodeParamControl → React component.
 *
 * | Control    | Component             | Renders                              |
 * |------------|-----------------------|--------------------------------------|
 * | select     | SelectControl         | Dropdown (enums)                     |
 * | switch     | SwitchControl         | Toggle (booleans)                    |
 * | slider     | SliderControl         | Range slider (bounded numbers)       |
 * | number     | NumberControl         | Number input (unbounded)             |
 * | text       | TextControl           | Text input (strings, fallback)       |
 * | textarea   | TextareaControl       | Multiline text (strings w/ hint)     |
 * | tagPicker  | TagPickerControl      | Multi-select combobox (string arrays)|
 * | keyValue   | KeyValueEditorControl | Key→value pairs (records)            |
 */
export const CONTROL_REGISTRY: Record<NodeParamControl, ComponentType<ControlProps>> = {
  select: SelectControl,
  switch: SwitchControl,
  slider: SliderControl,
  number: NumberControl,
  text: TextControl,
  textarea: TextareaControl,
  tagPicker: TagPickerControl,
  keyValue: KeyValueEditorControl,
};

export type { ControlProps } from "./types";
