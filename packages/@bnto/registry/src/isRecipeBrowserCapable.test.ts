import { describe, it, expect } from "vitest";
import type { Definition } from "@bnto/nodes";
import { isRecipeBrowserCapable } from "./isRecipeBrowserCapable";
import { definitionToRecipe } from "./definitionToRecipe";

const node = (id: string, type: string, nodes?: Definition[]) =>
  ({ id, type, parameters: {}, ...(nodes ? { nodes } : {}) }) as Definition;

const recipeOf = (...steps: Definition[]) =>
  definitionToRecipe({
    ...node("r", "group", [node("input", "input"), ...steps, node("output", "output")]),
    name: "R",
  } as Definition);

describe("isRecipeBrowserCapable", () => {
  it("is true when every step runs in the browser", () => {
    expect(isRecipeBrowserCapable(recipeOf(node("a", "image-compress")))).toBe(true);
  });

  it("is false for a CLI-only step at the root", () => {
    expect(isRecipeBrowserCapable(recipeOf(node("a", "shell-run")))).toBe(false);
  });

  it("looks inside containers, at any depth", () => {
    const nested = node("g", "group", [node("l", "loop", [node("a", "shell-run")])]);
    expect(isRecipeBrowserCapable(recipeOf(nested))).toBe(false);
    const fine = node("g", "group", [node("a", "image-compress")]);
    expect(isRecipeBrowserCapable(recipeOf(fine))).toBe(true);
  });
});
