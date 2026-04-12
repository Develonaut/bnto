/** WebApplication JSON-LD schema with free pricing signal. */

import type { BntoEntry } from "@/lib/bntoRegistry";

export function buildWebAppSchema(entry: BntoEntry, pageUrl: string) {
  return {
    "@context": "https://schema.org",
    "@type": "WebApplication",
    name: entry.h1,
    description: entry.description,
    url: pageUrl,
    applicationCategory: "UtilityApplication",
    operatingSystem: "Any",
    browserRequirements: "Requires a modern web browser",
    offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
    featureList: entry.features,
  };
}
