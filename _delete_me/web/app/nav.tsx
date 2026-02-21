"use client";

import Link from "next/link";
import { useTheme } from "next-themes";
import { ThemeToggle } from "@bnto/ui";

export function Nav() {
  const { theme, setTheme } = useTheme();

  return (
    <nav className="fixed top-0 z-50 flex w-full items-center justify-between bg-background/80 px-6 py-3 backdrop-blur">
      <Link href="/" className="text-lg font-bold">
        Bnto
      </Link>
      <ThemeToggle theme={theme} setTheme={setTheme} />
    </nav>
  );
}
