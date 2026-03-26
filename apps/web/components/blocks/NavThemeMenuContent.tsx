"use client";

import { AnimatedThemeToggle, Button, RotateCcwIcon, Text } from "@bnto/ui";

import { LightDirectionDial } from "./LightDirectionDial";

interface NavThemeMenuContentProps {
  lightAngle: number;
  setLightAngle: (angle: number) => void;
  themeName: string;
  isDefault: boolean;
  onReset: () => void;
}

export function NavThemeMenuContent({
  lightAngle,
  setLightAngle,
  themeName,
  isDefault,
  onReset,
}: NavThemeMenuContentProps) {
  return (
    <div className="flex flex-col items-center gap-3">
      <div className="flex w-full items-center gap-3">
        <AnimatedThemeToggle elevation="sm" />
        <Text size="sm" className="w-full font-medium">
          {themeName}
        </Text>
      </div>
      <LightDirectionDial lightAngle={lightAngle} setLightAngle={setLightAngle} />
      <Button variant="muted" onClick={onReset} disabled={isDefault} className="w-full">
        <RotateCcwIcon />
        Reset
      </Button>
    </div>
  );
}
