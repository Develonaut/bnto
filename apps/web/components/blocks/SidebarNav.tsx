/**
 * Sidebar navigation content — middle section of the app sidebar.
 *
 * Explore menu + My Recipes. No logo or footer — those are composed
 * by AppSidebar via SidebarShell slots.
 */

"use client";

import Link from "next/link";

import { BookOpenIcon, Label, Menu, MenuContent, MenuItem, MenuLabel, MenuTrigger } from "@bnto/ui";

import { NavButton } from "./NavButton";
import { RECIPES } from "./navData";

export function SidebarNav() {
  return (
    <>
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
    </>
  );
}
