/**
 * Recipe page content registry.
 *
 * Single source of truth for below-the-fold SEO content on recipe pages.
 * Category-level content is in categoryContent.ts (one block per category).
 * Recipe-specific FAQ overrides are optional and sparse.
 *
 * Adding a recipe (existing category): nothing to do.
 * Adding a new category: add one block to CATEGORY_CONTENT in categoryContent.ts.
 * Adding recipe-specific FAQ: add slug entry to RECIPE_FAQ below.
 */

import type { Recipe } from "@bnto/core";
import { CATEGORY_CONTENT } from "./categoryContent";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface Step {
  title: string;
  description: string;
}

export interface Feature {
  heading: string;
  body: string;
}

export interface QA {
  question: string;
  answer: string;
}

// ---------------------------------------------------------------------------
// Recipe-specific FAQ overrides (optional, sparse)
// ---------------------------------------------------------------------------

const RECIPE_FAQ: Record<string, QA[]> = {
  "compress-images": [
    {
      question: "How much can images be compressed?",
      answer:
        "Typical compression reduces file size by 40-80% depending on the source format and quality setting. PNG files with large flat-color areas see the biggest reductions. JPEG files with existing compression see smaller gains.",
    },
  ],
  "csv-to-json": [
    {
      question: "How are nested values handled?",
      answer:
        "Each CSV row becomes a flat JSON object. Column headers become keys and cell values become string values. Nested structures are not inferred automatically. For complex mappings, chain with a data transform node.",
    },
  ],
  "merge-csv": [
    {
      question: "Do all CSV files need the same columns?",
      answer:
        "No. The engine merges rows from all input files. If column headers differ, the output includes all unique columns. Missing values are left empty.",
    },
  ],
};

// ---------------------------------------------------------------------------
// Shared FAQ (applies to ALL recipes)
// ---------------------------------------------------------------------------

const SHARED_FAQ: QA[] = [
  {
    question: "Is this really free?",
    answer:
      "Yes. No signup, no watermarks, no daily caps, no quality reduction. The engine runs on your machine, so it costs nothing to operate. All recipes are free and unlimited.",
  },
  {
    question: "Are my files private?",
    answer:
      "Your files never leave your browser. Processing runs locally via WebAssembly. Nothing is uploaded to any server. You can verify this in your browser's network tab.",
  },
  {
    question: "Can I use this on mobile?",
    answer:
      "Yes. The browser-based tools work on any device with a modern browser, including phones and tablets. Performance depends on your device's processing power.",
  },
];

// ---------------------------------------------------------------------------
// Template functions
// ---------------------------------------------------------------------------

/** Build the 3-step "How it works" flow for a recipe. */
export function getHowItWorks(recipe: Recipe): Step[] {
  const categoryContent = CATEGORY_CONTENT[recipe.category];
  const processDescription =
    categoryContent?.processDescription ??
    "Your files are processed using a compiled Rust engine running as WebAssembly. Nothing is uploaded. Everything runs in your browser.";

  return [
    {
      title: "Drop your files",
      description: `Drag and drop your ${recipe.accept.label} into the browser. No signup, no upload. Files stay on your device.`,
    },
    {
      title: "Processing happens locally",
      description: processDescription,
    },
    {
      title: "Download results",
      description:
        "Preview the output and download your processed files. Originals are never modified. Run as many times as you like.",
    },
  ];
}

/** Get the 3 feature highlights for a category. */
export function getFeatures(category: string): Feature[] {
  return CATEGORY_CONTENT[category]?.features ?? [];
}

/**
 * Assemble FAQ items for a recipe page.
 * Order: recipe-specific (if any) -> category -> shared.
 */
export function getFaq(slug: string, category: string): QA[] {
  const recipeSpecific = RECIPE_FAQ[slug] ?? [];
  const categoryFaq = CATEGORY_CONTENT[category]?.faq ?? [];
  return [...recipeSpecific, ...categoryFaq, ...SHARED_FAQ];
}

// ---------------------------------------------------------------------------
// Category labels for BreadcrumbList
// ---------------------------------------------------------------------------

/** Human-readable category labels for breadcrumb display. */
export const CATEGORY_BREADCRUMB_LABELS: Record<string, string> = {
  image: "Image Tools",
  spreadsheet: "Data Tools",
  file: "File Tools",
  vector: "Vector Tools",
  video: "Video Tools",
};
