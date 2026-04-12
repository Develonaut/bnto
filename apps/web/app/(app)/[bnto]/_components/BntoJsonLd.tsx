/**
 * JSON-LD structured data for bnto tool pages.
 *
 * Renders multiple schemas: WebApplication, HowTo, FAQPage, BreadcrumbList.
 * All content reads from the recipePageContent registry.
 */

import type { Recipe } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { BASE_URL } from "@/lib/constants";
import { CATEGORY_BREADCRUMB_LABELS, getFaq, getHowItWorks } from "@/lib/recipePageContent";
import { buildWebAppSchema } from "./_schemas/buildWebAppSchema";
import { buildHowToSchema } from "./_schemas/buildHowToSchema";
import { buildFaqSchema } from "./_schemas/buildFaqSchema";
import { buildBreadcrumbSchema } from "./_schemas/buildBreadcrumbSchema";

interface BntoJsonLdProps {
  entry: BntoEntry;
  recipe?: Recipe;
}

export function BntoJsonLd({ entry, recipe }: BntoJsonLdProps) {
  const pageUrl = `${BASE_URL}/${entry.slug}`;
  const categoryLabel = CATEGORY_BREADCRUMB_LABELS[entry.category] ?? entry.category;
  const faqItems = getFaq(entry.slug, entry.category);
  const steps = recipe ? getHowItWorks(recipe) : [];

  const schemas = [
    buildWebAppSchema(entry, pageUrl),
    ...buildHowToSchema(entry, steps),
    ...buildFaqSchema(faqItems),
    buildBreadcrumbSchema(categoryLabel, entry, recipe, pageUrl),
  ];

  return (
    <>
      {schemas.map((schema, i) => (
        <script
          key={i}
          type="application/ld+json"
          dangerouslySetInnerHTML={{ __html: JSON.stringify(schema) }}
        />
      ))}
    </>
  );
}
