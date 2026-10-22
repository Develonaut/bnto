/** The OSes a recipe runs on, named the way people write them. */

import type { RecipeOs } from "@bnto/nodes";

const OS_LABELS: Record<RecipeOs, string> = {
  macos: "macOS",
  linux: "Linux",
  windows: "Windows",
};

export function recipeOsLabels(os: readonly RecipeOs[] | undefined): string[] {
  return (os ?? []).map((name) => OS_LABELS[name]);
}
