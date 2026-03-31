"use client";

import { Button, RadialSlider, SunIcon } from "@bnto/ui";

interface LightDirectionDialProps {
  lightAngle: number;
  setLightAngle: (angle: number) => void;
}

const angleToCardinal = (deg: number): string => {
  if (deg <= 141) return "NW";
  if (deg <= 158) return "NNW";
  if (deg <= 170) return "N";
  if (deg <= 190) return "N";
  if (deg <= 202) return "NNE";
  if (deg <= 219) return "NE";
  return "NE";
};

export function LightThumb({ isDragging }: { isDragging: boolean }) {
  return (
    <Button
      variant="warning"
      size="icon"
      elevation="sm"
      pressed={isDragging}
      className="pointer-events-none size-7"
    >
      <SunIcon className="size-3.5" />
    </Button>
  );
}

export function LightDirectionDial({ lightAngle, setLightAngle }: LightDirectionDialProps) {
  return (
    <div style={{ height: 80, clipPath: "inset(-12px -32px 0 -32px)" }}>
      <RadialSlider
        min={135}
        max={225}
        value={lightAngle}
        onChange={setLightAngle}
        startAngle={270}
        endAngle={90}
        size={128}
        strokeWidth={5}
        hideRing
        aria-label="Light direction"
        renderThumb={LightThumb}
      >
        <span className="text-xs font-mono font-medium text-muted-foreground">
          {angleToCardinal(lightAngle)}
        </span>
      </RadialSlider>
    </div>
  );
}
