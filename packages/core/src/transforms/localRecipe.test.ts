import { describe, it, expect } from "vitest";
import { draftToRecipeListItem } from "./localRecipe";
import type { LocalDraft } from "./localRecipe";

describe("draftToRecipeListItem", () => {
  const draft: LocalDraft = {
    definition: {
      nodes: [{ id: "n1" }, { id: "n2" }],
    },
    metadata: { id: "recipe-1", name: "My Recipe" },
    savedAt: 1710000000000,
  };

  it("maps id from metadata", () => {
    expect(draftToRecipeListItem(draft).id).toBe("recipe-1");
  });

  it("maps name from metadata", () => {
    expect(draftToRecipeListItem(draft).name).toBe("My Recipe");
  });

  it("counts nodes from definition", () => {
    expect(draftToRecipeListItem(draft).nodeCount).toBe(2);
  });

  it("maps savedAt to updatedAt", () => {
    expect(draftToRecipeListItem(draft).updatedAt).toBe(1710000000000);
  });

  it("handles missing nodes array", () => {
    const noNodes: LocalDraft = {
      ...draft,
      definition: {},
    };
    expect(draftToRecipeListItem(noNodes).nodeCount).toBe(0);
  });
});
