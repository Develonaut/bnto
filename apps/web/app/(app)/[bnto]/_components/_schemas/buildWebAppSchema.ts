/**
 * Application JSON-LD schema with free pricing signal. Browser recipes are a
 * WebApplication; CLI-only ones are SoftwareApplication, with no OS claimed
 * until recipes carry the OSes they run on.
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
  if (!entry.runsInBrowser) return { ...base, "@type": "SoftwareApplication" };
  return {
    ...base,
    "@type": "WebApplication",
    operatingSystem: "Any",
    browserRequirements: "Requires a modern web browser",
  };
}
