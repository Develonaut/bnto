"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@bnto/ui";
import { Button } from "@bnto/ui/button";
import { ThemeToggle } from "@bnto/ui/theme-toggle";
import { MobileNav } from "./MobileNav";
import { NavUser } from "./NavUser";
import { NAV_ITEMS } from "./nav-items";

/**
 * Floating glassmorphic pill navbar for the app shell.
 *
 * Fixed to the top center of the viewport with backdrop blur,
 * translucent background, and rounded pill shape.
 */
export function AppHeader() {
  const pathname = usePathname();

  return (
    <header className="fixed top-4 left-1/2 z-50 w-[min(90%,700px)] -translate-x-1/2">
      <div className="flex h-14 items-center gap-4 rounded-[2rem] border border-border/50 bg-background/70 px-4 shadow-md backdrop-blur-md">
        {/* Mobile menu */}
        <MobileNav />

        {/* Logo */}
        <Link href="/" className="flex items-center gap-2">
          <span className="font-display text-lg font-semibold">bnto</span>
        </Link>

        {/* Desktop navigation */}
        <nav className="ml-4 hidden items-center gap-1 md:flex">
          {NAV_ITEMS.map((item) => {
            const isActive = pathname === item.href;
            return (
              <Button
                key={item.href}
                variant="ghost"
                asChild
                className={cn("rounded-full", isActive && "bg-muted")}
              >
                <Link href={item.href}>{item.label}</Link>
              </Button>
            );
          })}
        </nav>

        {/* Right side */}
        <div className="ml-auto flex items-center gap-1">
          <ThemeToggle />
          <NavUser />
        </div>
      </div>
    </header>
  );
}
