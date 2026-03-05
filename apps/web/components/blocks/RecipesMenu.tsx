/**
 * Explorer dropdown menu — categorized recipe links from the engine menu.
 *
 * Used in both DesktopNav and MobileNavMenu.
 */

import Link from "next/link";

import { BookOpenIcon, Menu, MenuTrigger, MenuContent, MenuLabel, MenuItem } from "@bnto/ui";

import { RECIPES } from "./navData";

export function RecipesMenu() {
  return (
    <Menu>
      <MenuTrigger variant="outline" elevation="sm">
        <BookOpenIcon />
        Explore
      </MenuTrigger>
      <MenuContent className="w-[28rem] p-3" offset="lg">
        <ul className="grid grid-cols-2 gap-1">
          {RECIPES.map((category) => (
            <li key={category.title} className="col-span-2">
              <MenuLabel>{category.title}</MenuLabel>
              <ul className="grid grid-cols-2 gap-1">
                {category.links.map((link) => (
                  <li key={link.url}>
                    <MenuItem asChild className="flex-col items-start gap-1 py-2.5">
                      <Link href={link.url} className="no-underline">
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
  );
}
