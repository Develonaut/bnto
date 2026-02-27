import type { BntoEntry } from "@/lib/bntoRegistry";
import { BASE_URL } from "@/lib/constants";

/**
 * JSON-LD structured data for bnto tool pages.
 *
 * Renders a WebApplication schema with free pricing signal,
 * feature list, and browser-based operating system.
 */
export function BntoJsonLd({ entry }: { entry: BntoEntry }) {
  const jsonLd = {
    "@context": "https://schema.org",
    "@type": "WebApplication",
    name: entry.h1,
    description: entry.description,
    url: `${BASE_URL}/${entry.slug}`,
    applicationCategory: "UtilityApplication",
    operatingSystem: "Any",
    browserRequirements: "Requires a modern web browser",
    offers: {
      "@type": "Offer",
      price: "0",
      priceCurrency: "USD",
    },
    featureList: entry.features,
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
    />
  );
}
