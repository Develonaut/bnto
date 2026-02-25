"use client";

import { useCallback, useRef, useSyncExternalStore } from "react";
import { Moon, Sun, Sunset } from "lucide-react";
import { useTheme } from "next-themes";

import { Button } from "@/components/ui/button";

type ThemeName = "light" | "sunset" | "dark";

const THEME_CYCLE: ThemeName[] = ["light", "sunset", "dark"];

function nextTheme(current: ThemeName): ThemeName {
  const idx = THEME_CYCLE.indexOf(current);
  return THEME_CYCLE[(idx + 1) % THEME_CYCLE.length];
}

const noop = () => () => {};

export function AnimatedThemeToggle() {
  const { resolvedTheme, setTheme } = useTheme();
  const buttonRef = useRef<HTMLButtonElement>(null);
  const mounted = useSyncExternalStore(noop, () => true, () => false);

  // Before mount, resolvedTheme is undefined (SSR). Don't render a theme
  // attribute until mounted to avoid hydration mismatch. All icons start at
  // scale-0, so nothing shows until data-theme is set.
  const current = mounted ? ((resolvedTheme ?? "light") as ThemeName) : undefined;

  const toggleTheme = useCallback(async () => {
    if (!current) return;
    const target = nextTheme(current);

    // Reduced motion — instant switch, no animation
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      setTheme(target);
      return;
    }

    // Fallback for browsers without View Transitions API
    if (
      !document.startViewTransition ||
      !buttonRef.current
    ) {
      setTheme(target);
      return;
    }

    // Capture the old snapshot, apply the new theme synchronously,
    // then animate a circular clip-path reveal from the button center.
    // setTheme() from next-themes updates the DOM class directly (synchronous),
    // so no flushSync needed.
    const transition = document.startViewTransition(() => {
      setTheme(target);
    });

    await transition.ready;

    const { top, left, width, height } =
      buttonRef.current.getBoundingClientRect();
    const x = left + width / 2;
    const y = top + height / 2;

    // Max radius = distance from button center to the farthest viewport corner
    const maxRadius = Math.hypot(
      Math.max(x, window.innerWidth - x),
      Math.max(y, window.innerHeight - y),
    );

    document.documentElement.animate(
      {
        clipPath: [
          `circle(0px at ${x}px ${y}px)`,
          `circle(${maxRadius}px at ${x}px ${y}px)`,
        ],
      },
      {
        duration: 500,
        easing: "ease-in-out",
        pseudoElement: "::view-transition-new(root)",
      },
    );
  }, [current, setTheme]);

  return (
    <Button
      ref={buttonRef}
      variant="outline"
      size="icon"
      className="group size-9"
      data-theme={current}
      onClick={toggleTheme}
    >
      {/* All three icons rendered — CSS transitions the active one in */}
      <Sun className="absolute size-4 scale-0 rotate-90 transition-all group-data-[theme=light]:scale-100 group-data-[theme=light]:rotate-0" />
      <Sunset className="absolute size-4 scale-0 rotate-90 transition-all group-data-[theme=sunset]:scale-100 group-data-[theme=sunset]:rotate-0" />
      <Moon className="absolute size-4 scale-0 -rotate-90 transition-all group-data-[theme=dark]:scale-100 group-data-[theme=dark]:rotate-0" />
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}
