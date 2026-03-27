/**
 * CollectionPage JSON-LD for the explore page.
 *
 * Describes the explore page as an ItemList of recipe tool pages,
 * helping search engines understand the catalog structure.
 */

import { getAllRecipes } from "@bnto/registry";
import { BASE_URL } from "@/lib/constants";

export function ExploreJsonLd() {
  const recipes = getAllRecipes();

  const jsonLd = {
    "@context": "https://schema.org",
    "@type": "CollectionPage",
    name: "Explore Recipes",
    description:
      "Browse free browser-based recipes — compress images, clean CSVs, rename files, and more. No signup, no upload.",
    url: `${BASE_URL}/explore`,
    mainEntity: {
      "@type": "ItemList",
      itemListElement: recipes.map((r, i) => ({
        "@type": "ListItem",
        position: i + 1,
        url: `${BASE_URL}/${r.slug}`,
        name: r.name,
      })),
    },
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
    />
  );
}
