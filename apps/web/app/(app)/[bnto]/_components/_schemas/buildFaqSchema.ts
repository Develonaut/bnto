/** FAQPage JSON-LD schema for AI search citation. */

import type { QA } from "@/lib/recipePageContent";

export function buildFaqSchema(items: QA[]): object[] {
  if (items.length === 0) return [];

  return [
    {
      "@context": "https://schema.org",
      "@type": "FAQPage",
      mainEntity: items.map((item) => ({
        "@type": "Question",
        name: item.question,
        acceptedAnswer: { "@type": "Answer", text: item.answer },
      })),
    },
  ];
}
