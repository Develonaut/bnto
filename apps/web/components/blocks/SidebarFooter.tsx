/**
 * SidebarFooter — bottom section of the app sidebar.
 *
 * Pricing, FAQ, GitHub, and Beer links. Pinned to the bottom
 * of the SidebarShell via the footer slot.
 */

"use client";

import { BeerIcon, Button, GithubIcon, Row } from "@bnto/ui";

import { BUYMEACOFFEE_URL, GITHUB_URL } from "@/lib/copy";

import { NavButton } from "./NavButton";
import { PAGE_LINKS } from "./navData";

export function SidebarFooter() {
  return (
    <nav className="flex flex-col gap-2">
      {PAGE_LINKS.map((link) => (
        <NavButton
          key={link.href}
          href={link.href}
          className="w-full"
          data-testid={`nav-link-${link.href.replace("/", "")}`}
        >
          {link.label}
        </NavButton>
      ))}
      <Row className="gap-2 mt-1">
        <Button
          variant="secondary"
          href={GITHUB_URL}
          target="_blank"
          rel="noopener noreferrer"
          className="flex-1"
        >
          <GithubIcon />
          GitHub
        </Button>
        <Button
          variant="warning"
          href={BUYMEACOFFEE_URL}
          target="_blank"
          rel="noopener noreferrer"
          className="flex-1"
        >
          <BeerIcon />
          Beer
        </Button>
      </Row>
    </nav>
  );
}
