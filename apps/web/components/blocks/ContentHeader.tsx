/**
 * Content header — sits above the main scrollable area.
 *
 * Left side: hamburger toggle (mobile only).
 * Right side: GitHub, theme menu, user menu.
 */

"use client";

import { Badge, Button, GithubIcon, MenuIcon } from "@bnto/ui";

import { GITHUB_URL } from "@/lib/copy";

import { NavThemeMenu } from "./NavThemeMenu";
import { NavUser } from "./NavUser";

interface ContentHeaderProps {
  onToggleMobileMenu: () => void;
}

export function ContentHeader({ onToggleMobileMenu }: ContentHeaderProps) {
  return (
    <header className="sticky top-0 z-10 flex h-16 shrink-0 items-center justify-between border-b border-border bg-background px-6 lg:px-8">
      {/* Mobile menu toggle — mobile only */}
      <div className="lg:hidden">
        <Button
          variant="ghost"
          size="icon"
          onClick={onToggleMobileMenu}
          data-testid="mobile-menu-button"
          aria-label="Open menu"
        >
          <MenuIcon />
        </Button>
      </div>

      {/* Spacer on desktop (no hamburger) */}
      <div className="hidden lg:block" />

      {/* Right: action buttons */}
      <div className="flex items-center gap-2">
        <Button variant="primary" elevation="sm" href="/editor">
          Create <Badge variant="secondary">Beta</Badge>
        </Button>
        <Button
          variant="outline"
          size="icon"
          elevation="sm"
          href={GITHUB_URL}
          target="_blank"
          rel="noopener noreferrer"
          aria-label="GitHub"
        >
          <GithubIcon />
        </Button>
        <NavThemeMenu />
        <NavUser />
      </div>
    </header>
  );
}
