import { describe, expect, it } from "vitest";

import { addNode } from "./addNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { definitionToRecipe } from "./definitionToRecipe";

describe("definitionToRecipe", () => {
  it("wraps a definition into a complete recipe", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def);

    expect(recipe.slug).toBeTruthy();
    expect(recipe.name).toBeTruthy();
    expect(recipe.description).toBeTruthy();
    expect(recipe.category).toBeTruthy();
    expect(recipe.accept).toBeTruthy();
    expect(recipe.features).toEqual([]);
    expect(recipe.seo).toBeTruthy();
    expect(recipe.definition).toBe(def);
  });

  it("derives slug from definition name", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def);

    expect(recipe.slug).toBe("new-recipe");
  });

  it("handles special characters in slug generation", () => {
    const def = { ...createBlankDefinition(), name: "My Custom Recipe!!! (v2)" };
    const recipe = definitionToRecipe(def);

    expect(recipe.slug).toBe("my-custom-recipe-v2");
  });

  it("falls back to 'untitled' slug for empty name", () => {
    const def = { ...createBlankDefinition(), name: "" };
    const recipe = definitionToRecipe(def);

    expect(recipe.slug).toBe("untitled");
  });

  it("uses metadata overrides when provided", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def, {
      slug: "custom-slug",
      name: "Custom Name",
      description: "A custom description",
      category: "image",
      features: ["PNG", "JPEG"],
    });

    expect(recipe.slug).toBe("custom-slug");
    expect(recipe.name).toBe("Custom Name");
    expect(recipe.description).toBe("A custom description");
    expect(recipe.category).toBe("image");
    expect(recipe.features).toEqual(["PNG", "JPEG"]);
  });

  it("uses custom accept spec when provided", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def, {
      accept: {
        mimeTypes: ["image/jpeg", "image/png"],
        extensions: [".jpg", ".png"],
        label: "JPEG or PNG images",
      },
    });

    expect(recipe.accept.mimeTypes).toEqual(["image/jpeg", "image/png"]);
    expect(recipe.accept.label).toBe("JPEG or PNG images");
  });

  it("uses custom SEO spec when provided", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def, {
      seo: {
        title: "Custom Title -- bnto",
        h1: "Custom H1",
      },
    });

    expect(recipe.seo.title).toBe("Custom Title -- bnto");
    expect(recipe.seo.h1).toBe("Custom H1");
  });

  it("generates default SEO from the name", () => {
    const def = { ...createBlankDefinition(), name: "Compress Images" };
    const recipe = definitionToRecipe(def);

    expect(recipe.seo.title).toBe("Compress Images -- bnto");
    expect(recipe.seo.h1).toBe("Compress Images");
  });

  it("defaults to 'custom' category", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def);

    expect(recipe.category).toBe("custom");
  });

  it("defaults to accepting any files", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def);

    expect(recipe.accept.mimeTypes).toEqual([]);
    expect(recipe.accept.extensions).toEqual([]);
    expect(recipe.accept.label).toBe("Any files");
  });

  it("preserves the full definition tree including nodes", () => {
    const blank = createBlankDefinition();
    const { definition: withNodes } = addNode(blank, "image");
    const recipe = definitionToRecipe(withNodes);

    expect(recipe.definition.nodes).toHaveLength(1);
    expect(recipe.definition.nodes![0]!.type).toBe("image");
  });

  it("partial overrides don't affect unset fields", () => {
    const def = createBlankDefinition();
    const recipe = definitionToRecipe(def, { name: "Only Name" });

    // name is overridden
    expect(recipe.name).toBe("Only Name");
    // slug derived from overridden name
    expect(recipe.slug).toBe("only-name");
    // everything else is default
    expect(recipe.category).toBe("custom");
    expect(recipe.features).toEqual([]);
  });
});
