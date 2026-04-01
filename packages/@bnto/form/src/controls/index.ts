/**
 * Field control registry — maps NodeParamControl type to React component.
 *
 * This is the single lookup table that drives SchemaField rendering.
 * Adding a new control type = add a component file + add an entry here.
 */

import type { ComponentType } from "react";
import type { NodeParamControl } from "@bnto/core";
import type { ControlProps } from "./types";
import { FileControl } from "./FileControl";
import { KeyValueEditorControl } from "./KeyValueEditorControl";
import { NumberControl } from "./NumberControl";
import { PositionGridControl } from "./PositionGridControl";
import { SelectControl } from "./SelectControl";
import { SliderControl } from "./SliderControl";
import { SwitchControl } from "./SwitchControl";
import { TagPickerControl } from "./TagPickerControl";
import { TextControl } from "./TextControl";
import { TextareaControl } from "./TextareaControl";
import { WatermarkPreviewControl } from "./WatermarkPreviewControl";

/**
 * Registry: NodeParamControl → React component.
 *
 * | Control          | Component               | Renders                              |
 * |------------------|-------------------------|--------------------------------------|
 * | select           | SelectControl           | Dropdown (enums)                     |
 * | switch           | SwitchControl           | Toggle (booleans)                    |
 * | slider           | SliderControl           | Range slider (bounded numbers)       |
 * | number           | NumberControl           | Number input (unbounded)             |
 * | text             | TextControl             | Text input (strings, fallback)       |
 * | textarea         | TextareaControl         | Multiline text (strings w/ hint)     |
 * | tagPicker        | TagPickerControl        | Multi-select combobox (string arrays)|
 * | keyValue         | KeyValueEditorControl   | Key→value pairs (records)            |
 * | file             | FileControl             | File picker → base64 data URI        |
 * | positionGrid     | PositionGridControl     | 3×3 clickable position grid          |
 * | watermarkPreview | WatermarkPreviewControl | Live overlay composite preview       |
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
  file: FileControl,
  positionGrid: PositionGridControl,
  watermarkPreview: WatermarkPreviewControl,
};
