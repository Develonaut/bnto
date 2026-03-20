"use client";

import { useState } from "react";

import { Label, RadialSlider, Slider, Stack, Text } from "@bnto/ui";

const compressionPresets = [
  { value: 60, label: "Draft" },
  { value: 80, label: "Balanced" },
  { value: 100, label: "Maximum" },
];

export function SliderShowcase() {
  const [slider, setSlider] = useState([65]);
  const [presetSlider, setPresetSlider] = useState([50]);
  const [dial, setDial] = useState(75);
  const [halfDial, setHalfDial] = useState(60);

  return (
    <Stack className="gap-10">
      {/* Linear sliders */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Linear sliders
        </Text>
        <div className="grid grid-cols-2 gap-8">
          <Slider label="Quality" value={slider} onValueChange={setSlider} max={100} />
          <Slider
            label="Compression"
            value={presetSlider}
            onValueChange={setPresetSlider}
            min={1}
            max={100}
            presets={compressionPresets}
          />
        </div>
      </div>

      {/* Radial sliders */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Radial sliders
        </Text>
        <div className="grid grid-cols-3 gap-8">
          <Stack gap="sm" className="items-center">
            <Label>Radial</Label>
            <RadialSlider
              min={0}
              max={100}
              value={dial}
              onChange={setDial}
              startAngle={240}
              endAngle={120}
              size={180}
              aria-label="Quality"
            >
              <Text size="lg" mono className="font-semibold">
                {dial}%
              </Text>
            </RadialSlider>
          </Stack>
          <Stack gap="sm" className="items-center">
            <Label>Half-circle</Label>
            <RadialSlider
              min={0}
              max={100}
              value={halfDial}
              onChange={setHalfDial}
              startAngle={270}
              endAngle={90}
              size={180}
              hideRing
              aria-label="Half-circle gauge"
            >
              <Text size="lg" mono className="font-semibold">
                {halfDial}%
              </Text>
            </RadialSlider>
          </Stack>
        </div>
      </div>
    </Stack>
  );
}
