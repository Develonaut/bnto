/** BreadcrumbList JSON-LD schema: Home > Category > Recipe. */

import type { Recipe } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { BASE_URL } from "@/lib/constants";

export function buildBreadcrumbSchema(
  categoryLabel: string,
  entry: BntoEntry,
  recipe: Recipe | undefined,
  pageUrl: string,
) {
  return {
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    itemListElement: [
      { "@type": "ListItem", position: 1, name: "Home", item: BASE_URL },
      {
        "@type": "ListItem",
        position: 2,
        name: categoryLabel,
        item: `${BASE_URL}/explore?category=${entry.category}`,
      },
      {
        "@type": "ListItem",
        position: 3,
        name: recipe?.name ?? entry.h1,
        item: pageUrl,
      },
    ],
  };
}
