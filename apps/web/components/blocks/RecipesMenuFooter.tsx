/**
 * Sticky footer for the Explore dropdown — links to /explore for the full catalog.
 *
 * Rendered outside the scrollable area so it stays visible regardless of scroll position.
 */

import Link from "next/link";

import { Divider, MenuItem, Text } from "@bnto/ui";

export function RecipesMenuFooter() {
  return (
    <div className="px-3 pb-3">
      <Divider className="mb-2" />
      <MenuItem asChild className="justify-center">
        <Link href="/explore" className="no-underline">
          <Text size="sm" weight="medium" color="muted">
            Browse all recipes &rarr;
          </Text>
        </Link>
      </MenuItem>
    </div>
  );
}
