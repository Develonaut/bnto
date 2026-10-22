import { describe, expect, it } from "vitest";

import { recipeOsLabels } from "./recipeOsLabels";

describe("recipeOsLabels", () => {
  it("names each OS the way people write it, in order", () => {
    expect(recipeOsLabels(["macos", "linux", "windows"])).toEqual(["macOS", "Linux", "Windows"]);
  });

  it("is empty for a recipe that carries no OSes", () => {
    expect(recipeOsLabels(undefined)).toEqual([]);
  });
});
