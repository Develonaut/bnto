"use client";

import { useTheme } from "next-themes";
import { SunIcon, MoonIcon } from "../icons";
import { cn } from "../lib/utils";

/**
 * Sun/Moon toggle for switching between light and dark mode.
 *
 * Uses next-themes under the hood. Icon cross-fades via CSS transitions.
 */
export function ThemeToggle({ className }: { className?: string }) {
  const { resolvedTheme, setTheme } = useTheme();

  return (
    <button
      type="button"
      onClick={() => setTheme(resolvedTheme === "dark" ? "light" : "dark")}
      className={cn(
        "inline-flex size-9 items-center justify-center rounded-full text-muted-foreground motion-safe:transition-colors hover:text-foreground",
        className,
      )}
      aria-label="Toggle theme"
    >
      <SunIcon className="size-4 scale-100 rotate-0 motion-safe:transition-transform dark:scale-0 dark:-rotate-90" />
      <MoonIcon className="absolute size-4 scale-0 rotate-90 motion-safe:transition-transform dark:scale-100 dark:rotate-0" />
    </button>
  );
}
