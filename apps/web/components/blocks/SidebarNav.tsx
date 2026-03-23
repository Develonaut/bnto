/**
 * Sidebar navigation content — used by AppSidebar (desktop).
 *
 * Top: Explore (menu opens right), My Recipes section.
 * Bottom: Pricing, FAQ, GitHub, Beer.
 */

"use client";

import Link from "next/link";

import {
  BeerIcon,
  BookOpenIcon,
  Button,
  GithubIcon,
  Label,
  Menu,
  MenuContent,
  MenuItem,
  MenuLabel,
  MenuTrigger,
  Row,
  Spacer,
} from "@bnto/ui";

import { BUYMEACOFFEE_URL, GITHUB_URL } from "@/lib/copy";

import { NavButton } from "./NavButton";
import { PAGE_LINKS, RECIPES } from "./navData";

export function SidebarNav() {
  return (
    <>
      {/* Logo */}
      <NavButton
        href="/"
        className="mb-4 w-fit text-xl font-display font-black tracking-tighter"
        data-testid="nav-link-home"
      >
        bnto
      </NavButton>

      {/* Explore — menu opens to the right */}
      <nav className="flex flex-col gap-2">
        <Menu>
          <MenuTrigger
            variant="outline"
            elevation="sm"
            className="w-full"
            data-testid="explore-button"
          >
            <BookOpenIcon />
            Explore
          </MenuTrigger>
          <MenuContent
            className="w-[28rem] p-3"
            side="right"
            offset="xl"
            data-testid="explore-dropdown"
          >
            <ul className="grid grid-cols-2 gap-1">
              {RECIPES.map((category) => (
                <li key={category.title} className="col-span-2">
                  <MenuLabel>{category.title}</MenuLabel>
                  <ul className="grid grid-cols-2 gap-1">
                    {category.links.map((link) => (
                      <li key={link.url}>
                        <MenuItem asChild className="flex-col items-start gap-1 py-2.5">
                          <Link
                            href={link.url}
                            data-testid={`explore-link-${link.url.replace("/", "")}`}
                            className="no-underline"
                          >
                            <span className="text-sm leading-normal font-medium">{link.label}</span>
                            <span className="text-xs leading-normal text-muted-foreground">
                              {link.description}
                            </span>
                          </Link>
                        </MenuItem>
                      </li>
                    ))}
                  </ul>
                </li>
              ))}
            </ul>
          </MenuContent>
        </Menu>
      </nav>

      {/* My Recipes section */}
      <div className="mt-4 flex flex-col gap-2">
        <Label className="px-3 text-xs text-muted-foreground">My Recipes</Label>
        <NavButton href="/my-recipes" className="w-full" data-testid="nav-link-my-recipes">
          My Recipes
        </NavButton>
      </div>

      <Spacer />

      {/* Bottom — secondary nav + external links */}
      <nav className="flex flex-col gap-2 border-t border-border pt-3">
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
    </>
  );
}
