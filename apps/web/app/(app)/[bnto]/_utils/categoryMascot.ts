/** Maps recipe categories to mascot SVG paths. */

const CATEGORY_MASCOTS: Record<string, string> = {
  image: "/mascots/sushi-thumbsup.svg",
  spreadsheet: "/mascots/sushi-onigiri.svg",
  file: "/mascots/salmon-chopstick.svg",
  vector: "/mascots/penguin-chef.svg",
};

const FALLBACK = "/mascots/octopus-chef.svg";

export function getCategoryMascot(category: string): string {
  return CATEGORY_MASCOTS[category] ?? FALLBACK;
}
