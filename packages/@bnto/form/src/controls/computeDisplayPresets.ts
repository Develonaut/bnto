import type { SliderPreset } from "@bnto/ui";

/** Convert between stored and display values when inverted is set. */
export function invertValue(value: number, min: number, max: number): number {
  return min + max - value;
}

/** Map presets from stored-value space to display-value space, sorted ascending. */
export function computeDisplayPresets(
  presets: SliderPreset[] | undefined,
  inverted: boolean,
  min: number,
  max: number,
): SliderPreset[] | undefined {
  if (!presets) return undefined;
  if (!inverted) return [...presets].sort((a, b) => a.value - b.value);
  return presets
    .map((p) => ({ ...p, value: invertValue(p.value, min, max) }))
    .sort((a, b) => a.value - b.value);
}
