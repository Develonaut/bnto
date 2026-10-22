/**
 * Application JSON-LD schema with free pricing signal. Browser recipes are a
 * WebApplication; CLI-only ones are SoftwareApplication, claiming the OSes
 * the recipe has steps for.
 */

import type { BntoEntry } from "@/lib/bntoRegistry";

export function buildWebAppSchema(entry: BntoEntry, pageUrl: string) {
  const base = {
    "@context": "https://schema.org",
    name: entry.h1,
    description: entry.description,
    url: pageUrl,
    applicationCategory: "UtilityApplication",
    offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
    featureList: entry.features,
  };
  if (!entry.runsInBrowser) {
    return {
      ...base,
      "@type": "SoftwareApplication",
      operatingSystem: entry.operatingSystems.join(", ") || undefined,
    };
  }
  return {
    ...base,
    "@type": "WebApplication",
    operatingSystem: "Any",
    browserRequirements: "Requires a modern web browser",
  };
}
