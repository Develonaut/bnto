/**
 * 3-column featured recipe grid for the Explore dropdown menu.
 *
 * Shows a curated subset of recipes grouped by category.
 * The "Browse all" footer is in RecipesMenuFooter (sticky, outside scroll area).
 */

import Link from "next/link";

import { MenuItem, MenuLabel, Text } from "@bnto/ui";

import { FEATURED_RECIPES } from "./nav";

export function RecipesMenuGrid() {
  return (
    <ul className="grid grid-cols-3 gap-1">
      {FEATURED_RECIPES.map((category) => (
        <li key={category.title} className="col-span-3">
          <MenuLabel>{category.title}</MenuLabel>
          <ul className="grid grid-cols-3 gap-1">
            {category.links.map((link) => (
              <li key={link.url}>
                <MenuItem asChild className="flex-col items-start gap-1 py-2.5">
                  <Link
                    href={link.url}
                    data-testid={`explore-link-${link.url.replace("/", "")}`}
                    className="no-underline"
                  >
                    <Text size="sm" weight="medium" className="leading-normal">
                      {link.label}
                    </Text>
                    <Text size="xs" color="muted" className="leading-normal">
                      {link.description}
                    </Text>
                  </Link>
                </MenuItem>
              </li>
            ))}
          </ul>
        </li>
      ))}
    </ul>
  );
}
