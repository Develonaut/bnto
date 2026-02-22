/**
 * Bnto slug registry -- single source of truth for all predefined bnto URLs.
 *
 * Drives generateStaticParams, generateMetadata, sitemap, gallery,
 * llms.txt, and middleware slug validation from one place.
 */

export interface BntoEntry {
  slug: string;
  title: string;
  description: string;
  h1: string;
  fixture: string;
  features: string[];
}

/**
 * All registered Tier 1 bntos. Each entry maps to a public URL at /{slug}.
 */
export const BNTO_REGISTRY: readonly BntoEntry[] = [
  {
    slug: "compress-images",
    title: "Compress Images Online Free -- bnto",
    description:
      "Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.",
    h1: "Compress Images Online Free",
    fixture: "compress-images.bnto.json",
    features: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
  },
  {
    slug: "resize-images",
    title: "Resize Images Online Free -- bnto",
    description:
      "Resize images to exact dimensions or percentages. Free, no signup required.",
    h1: "Resize Images Online Free",
    fixture: "resize-images.bnto.json",
    features: ["PNG", "JPEG", "WebP", "Custom dimensions", "Browser-based"],
  },
  {
    slug: "convert-image-format",
    title: "Convert Image Format Online Free -- bnto",
    description:
      "Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.",
    h1: "Convert Image Format Online Free",
    fixture: "convert-image-format.bnto.json",
    features: ["PNG", "JPEG", "WebP", "GIF", "Browser-based"],
  },
  {
    slug: "rename-files",
    title: "Rename Files Online Free -- bnto",
    description:
      "Batch rename files with patterns. Free, no signup required.",
    h1: "Rename Files Online Free",
    fixture: "rename-files.bnto.json",
    features: ["Batch rename", "Pattern matching", "Browser-based"],
  },
  {
    slug: "clean-csv",
    title: "Clean CSV Online Free -- bnto",
    description:
      "Remove empty rows, trim whitespace, deduplicate CSV data. Free, no signup.",
    h1: "Clean CSV Online Free",
    fixture: "clean-csv.bnto.json",
    features: ["CSV", "Remove duplicates", "Trim whitespace", "Browser-based"],
  },
  {
    slug: "rename-csv-columns",
    title: "Rename CSV Columns Online Free -- bnto",
    description:
      "Rename CSV column headers in bulk. Free, no signup required.",
    h1: "Rename CSV Columns Online Free",
    fixture: "rename-csv-columns.bnto.json",
    features: ["CSV", "Column rename", "Bulk edit", "Browser-based"],
  },
] as const satisfies readonly BntoEntry[];

const slugSet = new Set(BNTO_REGISTRY.map((b) => b.slug));

/** Returns true if the slug maps to a registered bnto. */
export function isValidBntoSlug(slug: string): boolean {
  return slugSet.has(slug);
}

/** Returns the registry entry for a slug, or undefined if not found. */
export function getBntoBySlug(slug: string): BntoEntry | undefined {
  return BNTO_REGISTRY.find((b) => b.slug === slug);
}

/**
 * Paths reserved for app routes -- never use as bnto slugs.
 *
 * Validated at test time to prevent collisions between bnto slugs
 * and current or anticipated app routes.
 */
export const RESERVED_PATHS = [
  "signin",
  "signout",
  "signup",
  "waitlist",
  "workflows",
  "executions",
  "settings",
  "pricing",
  "about",
  "blog",
  "docs",
  "changelog",
  "api",
  "admin",
  "dashboard",
] as const;
