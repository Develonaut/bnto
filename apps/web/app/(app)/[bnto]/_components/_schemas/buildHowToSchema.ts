/** HowTo JSON-LD schema derived from recipe steps. */

import type { BntoEntry } from "@/lib/bntoRegistry";
import type { Step } from "@/lib/recipePageContent";

export function buildHowToSchema(entry: BntoEntry, steps: Step[]): object[] {
  if (steps.length === 0) return [];

  return [
    {
      "@context": "https://schema.org",
      "@type": "HowTo",
      name: entry.h1,
      description: entry.description,
      step: steps.map((s, i) => ({
        "@type": "HowToStep",
        position: i + 1,
        name: s.title,
        text: s.description,
      })),
    },
  ];
}
