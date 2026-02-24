"use client";

import { Sun } from "lucide-react";

import { Slider } from "@/components/ui/slider";
import { Text } from "@/components/ui/Text";

interface LightSourceSliderProps {
  value: number;
  onChange: (value: number) => void;
}

export function LightSourceSlider({ value, onChange }: LightSourceSliderProps) {
  return (
    <div className="flex items-center gap-4">
      <Sun className="size-5 shrink-0 text-warning" />
      <Slider
        min={135}
        max={225}
        step={1}
        value={[value]}
        onValueChange={([v]) => onChange(v)}
        className="flex-1"
      />
      <Text size="sm" className="w-12 text-right font-mono tabular-nums">
        {value}&deg;
      </Text>
    </div>
  );
}
