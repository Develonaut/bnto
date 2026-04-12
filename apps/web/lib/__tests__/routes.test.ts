import { describe, expect, it } from "vitest";
import { editorUrl, ROUTES } from "../routes";

describe("ROUTES", () => {
  it("contains all expected route paths", () => {
    expect(ROUTES.home).toBe("/");
    expect(ROUTES.explore).toBe("/explore");
    expect(ROUTES.editor).toBe("/editor");
  });

  it("has exactly 3 routes", () => {
    expect(Object.keys(ROUTES)).toHaveLength(3);
  });
});

describe("editorUrl", () => {
  it("returns /editor?recipe={id}", () => {
    expect(editorUrl("abc-123")).toBe("/editor?recipe=abc-123");
  });
});
