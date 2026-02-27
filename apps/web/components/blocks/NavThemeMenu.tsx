"use client";

import { useTheme } from "@/components/useTheme";

import { RotateCcwIcon, SunIcon } from "@/components/ui/icons";

import { AnimatedThemeToggle } from "@/components/ui/AnimatedThemeToggle";
import { Button } from "@/components/ui/Button";
import { Menu } from "@/components/ui/Menu";
import { RadialSlider } from "@/components/ui/RadialSlider";
import { Text } from "@/components/ui/Text";
import {
  THEME_STORE_DEFAULT_ANGLE,
  useThemeStore,
} from "@/lib/stores/themeStore";

/* ── Helpers ──────────────────────────────────────────────────── */

const THEME_NAMES: Record<string, string> = {
  light: "Los Angeles",
  sunset: "Monaco",
  dark: "Tokyo",
};

function angleToCardinal(deg: number): string {
  if (deg <= 141) return "NW";
  if (deg <= 158) return "NNW";
  if (deg <= 170) return "N";
  if (deg <= 190) return "N";
  if (deg <= 202) return "NNE";
  if (deg <= 219) return "NE";
  return "NE";
}

function ThemeName() {
  const { resolvedTheme } = useTheme();
  return <>{THEME_NAMES[resolvedTheme ?? "light"] ?? "Los Angeles"}</>;
}

/* ── NavThemeMenu ─────────────────────────────────────────────── */

export function NavThemeMenu() {
  const lightAngle = useThemeStore((s) => s.lightAngle);
  const setLightAngle = useThemeStore((s) => s.setLightAngle);
  const { setTheme, resolvedTheme } = useTheme();

  const isDefault =
    lightAngle === THEME_STORE_DEFAULT_ANGLE && resolvedTheme === "light";

  const handleReset = () => {
    setLightAngle(THEME_STORE_DEFAULT_ANGLE);
    setTheme("light");
  };

  return (
    <Menu>
      <Menu.Trigger variant="outline" size="icon" elevation="sm">
        <SunIcon />
        <span className="sr-only">Theme settings</span>
      </Menu.Trigger>
      <Menu.Content className="w-auto p-4" offset="lg">
        <div className="flex flex-col items-center gap-3">
          {/* Theme toggle + city name */}
          <div className="flex w-full items-center gap-3">
            <AnimatedThemeToggle elevation="sm" />
            <Text size="sm" className="w-full font-medium">
              <ThemeName />
            </Text>
          </div>
          {/* Light direction dial */}
          <RadialSlider
            min={135}
            max={225}
            value={lightAngle}
            onChange={setLightAngle}
            startAngle={270}
            endAngle={90}
            size={128}
            strokeWidth={5}
            aria-label="Light direction"
            renderThumb={({ isDragging }) => (
              <Button
                variant="warning"
                size="icon"
                elevation="sm"
                pressed={isDragging}
                className="pointer-events-none size-7"
              >
                <SunIcon className="size-3.5" />
              </Button>
            )}
          >
            <span className="text-xs font-mono font-medium text-muted-foreground">
              {angleToCardinal(lightAngle)}
            </span>
          </RadialSlider>
          {/* Reset — always visible, disabled when at default */}
          <Button
            variant="muted"
            size="md"
            onClick={handleReset}
            disabled={isDefault}
            className="w-full"
          >
            <RotateCcwIcon />
            Reset
          </Button>
        </div>
      </Menu.Content>
    </Menu>
  );
}
