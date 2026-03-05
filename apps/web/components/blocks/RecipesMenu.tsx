/**
 * Explorer dropdown menu — categorized recipe links from the engine menu.
 *
 * Used in both DesktopNav and MobileNavMenu.
 */

import Link from "next/link";

import { BookOpenIcon, Menu } from "@bnto/ui";

import { RECIPES } from "./navData";

export function RecipesMenu() {
  return (
    <Menu>
      <Menu.Trigger variant="outline" elevation="sm">
        <BookOpenIcon />
        Explore
      </Menu.Trigger>
      <Menu.Content className="w-[28rem] p-3" offset="lg">
        <ul className="grid grid-cols-2 gap-1">
          {RECIPES.map((category) => (
            <li key={category.title} className="col-span-2">
              <Menu.Label>{category.title}</Menu.Label>
              <ul className="grid grid-cols-2 gap-1">
                {category.links.map((link) => (
                  <li key={link.url}>
                    <Menu.Item asChild className="flex-col items-start gap-1 py-2.5">
                      <Link href={link.url} className="no-underline">
                        <span className="text-sm leading-normal font-medium">
                          {link.label}
                        </span>
                        <span className="text-xs leading-normal text-muted-foreground">
                          {link.description}
                        </span>
                      </Link>
                    </Menu.Item>
                  </li>
                ))}
              </ul>
            </li>
          ))}
        </ul>
      </Menu.Content>
    </Menu>
  );
}
