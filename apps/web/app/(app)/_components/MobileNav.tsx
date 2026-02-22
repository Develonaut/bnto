"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn, MenuIcon } from "@bnto/ui";
import { Button } from "@bnto/ui/button";
import { Sheet } from "@bnto/ui/sheet";
import { NAV_ITEMS } from "./nav-items";

/**
 * Mobile navigation sheet.
 *
 * Triggered by the hamburger button visible on small screens.
 * Shows the same nav items as the desktop top bar.
 */
export function MobileNav() {
  const pathname = usePathname();

  return (
    <Sheet>
      <Sheet.Trigger asChild>
        <Button variant="ghost" size="icon" className="md:hidden">
          <MenuIcon className="size-5" />
          <span className="sr-only">Open menu</span>
        </Button>
      </Sheet.Trigger>
      <Sheet.Content side="left" className="w-72">
        <Sheet.Header>
          <Sheet.Title className="flex items-center gap-2">
            <span className="font-semibold">bnto</span>
          </Sheet.Title>
        </Sheet.Header>
        <nav className="mt-6 flex flex-col gap-1">
          {NAV_ITEMS.map((item) => {
            const Icon = item.icon;
            const isActive = pathname === item.href;
            return (
              <Link
                key={item.href}
                href={item.href}
                className={cn(
                  "flex items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-muted",
                  isActive && "bg-muted font-medium",
                )}
              >
                <Icon className="size-4" />
                {item.label}
              </Link>
            );
          })}
        </nav>
      </Sheet.Content>
    </Sheet>
  );
}
