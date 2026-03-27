/**
 * /explore — Recipe discovery page.
 *
 * Server component page with client leaf components for search,
 * filtering, and virtualized grid rendering.
 */

import type { Metadata } from "next";
import { Suspense } from "react";
import { AppShellContent } from "@bnto/ui";
import { ExploreHeader } from "./_components/ExploreHeader";
import { ExploreFilters } from "./_components/ExploreFilters";
import { ExploreRecipeGrid } from "./_components/ExploreRecipeGrid";
import { ExploreJsonLd } from "./_components/ExploreJsonLd";

export const metadata: Metadata = {
  title: "Explore Recipes",
  description:
    "Browse free browser-based recipes — compress images, clean CSVs, rename files, and more. No signup, no upload.",
  openGraph: {
    title: "Explore Recipes — bnto",
    description:
      "Browse free browser-based recipes — compress images, clean CSVs, rename files, and more.",
  },
};

export default function ExplorePage() {
  return (
    <>
      <ExploreJsonLd />
      <AppShellContent gap="sm">
        <ExploreHeader />
        <Suspense>
          <ExploreFilters />
          <ExploreRecipeGrid />
        </Suspense>
      </AppShellContent>
    </>
  );
}
