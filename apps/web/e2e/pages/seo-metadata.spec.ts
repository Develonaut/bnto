import { test, expect } from "../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * SEO metadata E2E tests.
 *
 * Verifies that each Tier 1 bnto slug renders correct metadata
 * in the HTML source — title, meta description, OG tags, h1,
 * and JSON-LD structured data. These must be server-rendered
 * (present in the initial HTML) for search engine indexing.
 */

/** All Tier 1 slugs with their expected SEO values. */
const TIER_1_SLUGS = [
  {
    slug: "compress-images",
    title: "Compress Images Online Free -- bnto",
    description:
      "Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.",
    h1: "Compress Images Online Free",
    features: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
  },
  {
    slug: "resize-images",
    title: "Resize Images Online Free -- bnto",
    description:
      "Resize images to exact dimensions or percentages. Free, no signup required.",
    h1: "Resize Images Online Free",
    features: [
      "PNG",
      "JPEG",
      "WebP",
      "Custom dimensions",
      "Browser-based",
    ],
  },
  {
    slug: "convert-image-format",
    title: "Convert Image Format Online Free -- bnto",
    description:
      "Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.",
    h1: "Convert Image Format Online Free",
    features: ["PNG", "JPEG", "WebP", "GIF", "Browser-based"],
  },
  {
    slug: "rename-files",
    title: "Rename Files Online Free -- bnto",
    description:
      "Batch rename files with patterns. Free, no signup required.",
    h1: "Rename Files Online Free",
    features: ["Batch rename", "Pattern matching", "Browser-based"],
  },
  {
    slug: "clean-csv",
    title: "Clean CSV Online Free -- bnto",
    description:
      "Remove empty rows, trim whitespace, deduplicate CSV data. Free, no signup.",
    h1: "Clean CSV Online Free",
    features: [
      "CSV",
      "Remove duplicates",
      "Trim whitespace",
      "Browser-based",
    ],
  },
  {
    slug: "rename-csv-columns",
    title: "Rename CSV Columns Online Free -- bnto",
    description:
      "Rename CSV column headers in bulk. Free, no signup required.",
    h1: "Rename CSV Columns Online Free",
    features: ["CSV", "Column rename", "Bulk edit", "Browser-based"],
  },
] as const;

test.describe("SEO metadata — all Tier 1 slugs", () => {
  for (const entry of TIER_1_SLUGS) {
    test(`/${entry.slug}: correct title and meta description`, async ({
      page,
    }) => {
      await page.goto(`/${entry.slug}`);

      // Title tag uses the absolute title from the registry (bypasses template).
      // Tool page titles already include the "-- bnto" brand suffix.
      await expect(page).toHaveTitle(entry.title);

      // Meta description present and correct
      const metaDesc = page.locator('meta[name="description"]');
      await expect(metaDesc).toHaveAttribute("content", entry.description);
    });

    test(`/${entry.slug}: OG tags present`, async ({ page }) => {
      await page.goto(`/${entry.slug}`);

      const ogTitle = page.locator('meta[property="og:title"]');
      await expect(ogTitle).toHaveAttribute("content", entry.title);

      const ogDesc = page.locator('meta[property="og:description"]');
      await expect(ogDesc).toHaveAttribute("content", entry.description);
    });

    test(`/${entry.slug}: h1 matches target query`, async ({ page }) => {
      await page.goto(`/${entry.slug}`);

      await expect(
        page.getByRole("heading", { level: 1, name: entry.h1 }),
      ).toBeVisible();
    });

    test(`/${entry.slug}: JSON-LD structured data`, async ({ page }) => {
      await page.goto(`/${entry.slug}`);

      // Extract JSON-LD from the page
      const jsonLd = await page.evaluate(() => {
        const script = document.querySelector(
          'script[type="application/ld+json"]',
        );
        return script ? JSON.parse(script.textContent ?? "{}") : null;
      });

      expect(jsonLd).not.toBeNull();
      expect(jsonLd["@context"]).toBe("https://schema.org");
      expect(jsonLd["@type"]).toBe("WebApplication");
      expect(jsonLd.name).toBe(entry.h1);
      expect(jsonLd.description).toBe(entry.description);
      expect(jsonLd.applicationCategory).toBe("UtilityApplication");
      expect(jsonLd.offers.price).toBe("0");
      expect(jsonLd.offers.priceCurrency).toBe("USD");
      expect(jsonLd.featureList).toEqual([...entry.features]);
    });
  }
});

test.describe("SEO — 404 for unknown slugs", () => {
  test("unknown slug returns 404 page", async ({ page }) => {
    const response = await page.goto("/not-a-real-bnto-tool");

    expect(response?.status()).toBe(404);

    await expect(
      page.getByRole("heading", { name: /Page Not Found/i }),
    ).toBeVisible();
  });
});

test.describe("SEO — canonical slug enforcement", () => {
  test("uppercase slug redirects to lowercase", async ({ page }) => {
    await page.goto("/Compress-Images");

    // Should redirect to the canonical lowercase slug
    await expect(page).toHaveURL(/\/compress-images$/);
  });

  test("underscore slug redirects to hyphenated", async ({ page }) => {
    await page.goto("/compress_images");

    // Should redirect to the canonical hyphenated slug
    await expect(page).toHaveURL(/\/compress-images$/);
  });
});
