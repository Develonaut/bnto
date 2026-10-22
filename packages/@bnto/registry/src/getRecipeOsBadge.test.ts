import { describe, expect, it } from "vitest";

import { getRecipeOsBadge } from "./getRecipeOsBadge";
import type { Recipe } from "./recipe";
import { RECIPES } from "./recipesCatalog";

function recipe(slug: string) {
  const found = RECIPES.find((r) => r.slug === slug);
  if (!found) throw new Error(`no recipe ${slug}`);
  return found;
}

/** `base` with its `os` replaced. */
const withOs = (base: Recipe, os: Recipe["os"]): Recipe => ({ ...base, os });

describe("getRecipeOsBadge", () => {
  it("names the one OS keep-awake runs on", () => {
    expect(getRecipeOsBadge(recipe("keep-awake"))).toBe("macOS and Linux only");
  });

  it("names every OS a recipe is limited to", () => {
    const twoOs = withOs(recipe("keep-awake"), ["macos", "linux"]);
    expect(getRecipeOsBadge(twoOs)).toBe("macOS and Linux only");
  });

  it("shows nothing for a CLI recipe that runs everywhere", () => {
    expect(getRecipeOsBadge(recipe("download-video"))).toBeNull();
  });

  it("shows nothing for a browser recipe, whatever its OSes", () => {
    expect(getRecipeOsBadge(recipe("compress-images"))).toBeNull();
    expect(getRecipeOsBadge(withOs(recipe("compress-images"), ["macos"]))).toBeNull();
  });

  it("shows nothing for a recipe whose OSes aren't known", () => {
    expect(getRecipeOsBadge(withOs(recipe("keep-awake"), undefined))).toBeNull();
  });
});
