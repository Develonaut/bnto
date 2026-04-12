/** Maps recipe categories to mascot SVG paths. */

const CATEGORY_MASCOTS: Record<string, string> = {
  image: "/mascots/sushi-thumbsup.svg",
  spreadsheet: "/mascots/octopus-chef.svg",
  file: "/mascots/sumo-sushi.svg",
  vector: "/mascots/penguin-chef.svg",
};

const FALLBACK = "/mascots/sumo-sushi.svg";

export function getCategoryMascot(category: string): string {
  return CATEGORY_MASCOTS[category] ?? FALLBACK;
}
