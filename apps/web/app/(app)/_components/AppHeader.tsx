"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn, SearchIcon } from "@bnto/ui";
import { Button } from "@bnto/ui/button";
import { Input } from "@bnto/ui/input";
import { MobileNav } from "./MobileNav";
import { NavUser } from "./NavUser";
import { NAV_ITEMS } from "./nav-items";

/**
 * Top navigation bar for authenticated app shell.
 *
 * Desktop: logo + flat nav links + search + user dropdown.
 * Mobile: hamburger → Sheet + user dropdown.
 */
export function AppHeader() {
  const pathname = usePathname();

  return (
    <header className="sticky top-0 z-50 border-b bg-background">
      <div className="flex h-14 items-center gap-4 px-4 lg:px-6">
        {/* Mobile menu */}
        <MobileNav />

        {/* Logo */}
        <Link href="/" className="flex items-center gap-2">
          <span className="text-lg font-semibold">bnto</span>
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
                className={cn(isActive && "bg-muted")}
              >
                <Link href={item.href}>{item.label}</Link>
              </Button>
            );
          })}
        </nav>

        {/* Right side */}
        <div className="ml-auto flex items-center gap-2">
          <div className="relative hidden md:block">
            <SearchIcon className="absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="search"
              placeholder="Search..."
              className="h-9 w-64 pl-8"
            />
          </div>
          <Button variant="ghost" size="icon" className="md:hidden">
            <SearchIcon className="size-5" />
            <span className="sr-only">Search</span>
          </Button>
          <NavUser />
        </div>
      </div>
    </header>
  );
}
