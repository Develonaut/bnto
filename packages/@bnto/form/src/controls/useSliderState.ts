import { useCallback, useMemo } from "react";
import type { SliderPreset } from "@bnto/ui";
import { invertValue, computeDisplayPresets } from "./computeDisplayPresets";

interface SliderStateOptions {
  min: number;
  max: number;
  inverted: boolean;
  value: unknown;
  presets?: SliderPreset[];
  onChange: (value: unknown) => void;
}

/** Encapsulates slider value inversion, preset mapping, and change handling. */
function useSliderState({ min, max, inverted, value, presets, onChange }: SliderStateOptions) {
  const stored = typeof value === "number" ? value : min;
  const display = inverted ? invertValue(stored, min, max) : stored;
  const displayPresets = useMemo(
    () => computeDisplayPresets(presets, inverted, min, max),
    [presets, inverted, min, max],
  );
  const sliderValue = useMemo(() => [display], [display]);
  const handleChange = useCallback(
    (vs: number[]) => onChange(inverted ? invertValue(vs[0]!, min, max) : vs[0]!),
    [onChange, inverted, min, max],
  );
  return { sliderValue, displayPresets, handleChange };
}

export { useSliderState };
