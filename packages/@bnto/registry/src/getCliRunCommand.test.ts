import { describe, expect, it } from "vitest";

import { getCliRunCommand } from "./getCliRunCommand";
import { RECIPES } from "./recipesCatalog";

function recipe(slug: string) {
  const found = RECIPES.find((r) => r.slug === slug);
  if (!found) throw new Error(`no recipe ${slug}`);
  return found;
}

describe("getCliRunCommand", () => {
  it("takes no argument for a recipe without input", () => {
    expect(getCliRunCommand(recipe("keep-awake"))).toBe("bnto run keep-awake");
  });

  it("takes a URL for a URL recipe", () => {
    expect(getCliRunCommand(recipe("download-video"))).toBe("bnto run download-video <url>");
  });

  it("takes files for a file recipe", () => {
    expect(getCliRunCommand(recipe("compress-images"))).toBe("bnto run compress-images <files...>");
  });
});
