/**
 * Recipes dropdown menu — categorized recipe links from the engine menu.
 *
 * Used in both DesktopNav and MobileNavMenu.
 */

import Link from "next/link";

import { BookOpenIcon } from "@/components/ui/icons";
import { Menu } from "@/components/ui/Menu";

import { RECIPES } from "./navData";

export function RecipesMenu() {
  return (
    <Menu>
      <Menu.Trigger variant="outline" elevation="sm">
        <BookOpenIcon />
        Recipes
      </Menu.Trigger>
      <Menu.Content className="w-[28rem] p-3" offset="lg">
        <ul className="grid grid-cols-2 gap-1">
          {RECIPES.map((category) => (
            <li key={category.title} className="col-span-2">
              <div className="px-3 pt-3 pb-1 text-xs font-medium uppercase tracking-wider text-muted-foreground">
                {category.title}
              </div>
              <ul className="grid grid-cols-2 gap-1">
                {category.links.map((link) => (
                  <li key={link.url}>
                    <Link
                      href={link.url}
                      className="flex flex-col gap-1 rounded-lg px-3 py-2.5 no-underline outline-hidden transition-colors select-none hover:bg-muted focus:bg-muted"
                    >
                      <span className="text-sm leading-normal font-medium">
                        {link.label}
                      </span>
                      <span className="text-muted-foreground text-xs leading-normal">
                        {link.description}
                      </span>
                    </Link>
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
