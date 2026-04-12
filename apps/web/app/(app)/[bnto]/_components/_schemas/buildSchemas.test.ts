/**
 * Unit tests for JSON-LD schema builder functions.
 *
 * These pure functions produce structured data critical for SEO.
 * Each test verifies @type, @context, output shape, and edge cases.
 */

import { describe, expect, it } from "vitest";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { QA, Step } from "@/lib/recipePageContent";
import { buildBreadcrumbSchema } from "./buildBreadcrumbSchema";
import { buildFaqSchema } from "./buildFaqSchema";
import { buildHowToSchema } from "./buildHowToSchema";
import { buildWebAppSchema } from "./buildWebAppSchema";

const ENTRY: BntoEntry = {
  slug: "compress-images",
  title: "Compress Images Online Free -- bnto",
  description: "Compress images without losing quality",
  h1: "Compress Images Online Free",
  fixture: "small.jpg",
  features: ["Lossless compression", "Batch processing"],
  category: "image",
};

const PAGE_URL = "https://bnto.io/compress-images";

describe("buildWebAppSchema", () => {
  it("returns a valid WebApplication schema", () => {
    const schema = buildWebAppSchema(ENTRY, PAGE_URL);

    expect(schema["@context"]).toBe("https://schema.org");
    expect(schema["@type"]).toBe("WebApplication");
    expect(schema.name).toBe(ENTRY.h1);
    expect(schema.description).toBe(ENTRY.description);
    expect(schema.url).toBe(PAGE_URL);
    expect(schema.offers).toEqual({ "@type": "Offer", price: "0", priceCurrency: "USD" });
    expect(schema.featureList).toEqual(ENTRY.features);
  });
});

describe("buildHowToSchema", () => {
  const STEPS: Step[] = [
    { title: "Upload", description: "Drop your files" },
    { title: "Configure", description: "Set quality" },
    { title: "Download", description: "Get results" },
  ];

  it("returns a single-element array with a HowTo schema", () => {
    const result = buildHowToSchema(ENTRY, STEPS);

    expect(result).toHaveLength(1);
    const schema = result[0] as Record<string, unknown>;
    expect(schema["@context"]).toBe("https://schema.org");
    expect(schema["@type"]).toBe("HowTo");
    expect(schema.name).toBe(ENTRY.h1);
  });

  it("maps steps to HowToStep with 1-based positions", () => {
    const result = buildHowToSchema(ENTRY, STEPS);
    const schema = result[0] as Record<string, unknown>;
    const steps = schema.step as Array<Record<string, unknown>>;

    expect(steps).toHaveLength(3);
    expect(steps[0]).toMatchObject({ "@type": "HowToStep", position: 1, name: "Upload" });
    expect(steps[2]).toMatchObject({ "@type": "HowToStep", position: 3, name: "Download" });
  });

  it("returns empty array when steps are empty", () => {
    expect(buildHowToSchema(ENTRY, [])).toEqual([]);
  });
});

describe("buildFaqSchema", () => {
  const ITEMS: QA[] = [
    { question: "Is it free?", answer: "Yes, always." },
    { question: "Is it safe?", answer: "Files never leave your browser." },
  ];

  it("returns a single-element array with a FAQPage schema", () => {
    const result = buildFaqSchema(ITEMS);

    expect(result).toHaveLength(1);
    const schema = result[0] as Record<string, unknown>;
    expect(schema["@context"]).toBe("https://schema.org");
    expect(schema["@type"]).toBe("FAQPage");
  });

  it("maps items to Question/Answer pairs", () => {
    const result = buildFaqSchema(ITEMS);
    const schema = result[0] as Record<string, unknown>;
    const entities = schema.mainEntity as Array<Record<string, unknown>>;

    expect(entities).toHaveLength(2);
    expect(entities[0]).toMatchObject({ "@type": "Question", name: "Is it free?" });
    expect((entities[0].acceptedAnswer as Record<string, unknown>).text).toBe("Yes, always.");
  });

  it("returns empty array when items are empty", () => {
    expect(buildFaqSchema([])).toEqual([]);
  });
});

describe("buildBreadcrumbSchema", () => {
  it("returns a BreadcrumbList with 3 items", () => {
    const recipe = { name: "Compress Images" } as Parameters<typeof buildBreadcrumbSchema>[2];
    const schema = buildBreadcrumbSchema("Image Tools", ENTRY, recipe, PAGE_URL);

    expect(schema["@context"]).toBe("https://schema.org");
    expect(schema["@type"]).toBe("BreadcrumbList");
    expect(schema.itemListElement).toHaveLength(3);
  });

  it("uses recipe name when available", () => {
    const recipe = { name: "Compress Images" } as Parameters<typeof buildBreadcrumbSchema>[2];
    const schema = buildBreadcrumbSchema("Image Tools", ENTRY, recipe, PAGE_URL);
    const items = schema.itemListElement as Array<Record<string, unknown>>;

    expect(items[2].name).toBe("Compress Images");
  });

  it("falls back to entry.h1 when recipe is undefined", () => {
    const schema = buildBreadcrumbSchema("Image Tools", ENTRY, undefined, PAGE_URL);
    const items = schema.itemListElement as Array<Record<string, unknown>>;

    expect(items[2].name).toBe(ENTRY.h1);
  });

  it("builds correct category explore URL", () => {
    const schema = buildBreadcrumbSchema("Image Tools", ENTRY, undefined, PAGE_URL);
    const items = schema.itemListElement as Array<Record<string, unknown>>;

    expect(items[1].name).toBe("Image Tools");
    expect((items[1].item as string).includes("category=image")).toBe(true);
  });

  it("uses 1-based positions", () => {
    const schema = buildBreadcrumbSchema("Image Tools", ENTRY, undefined, PAGE_URL);
    const items = schema.itemListElement as Array<Record<string, unknown>>;

    expect(items[0].position).toBe(1);
    expect(items[1].position).toBe(2);
    expect(items[2].position).toBe(3);
  });
});
