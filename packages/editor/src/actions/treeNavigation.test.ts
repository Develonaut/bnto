import { describe, it, expect } from "vitest";
import { resolveTreeNav } from "./treeNavigation";

const ids = ["a", "b", "c", "d"];

describe("resolveTreeNav", () => {
  it("returns null for empty list", () => {
    expect(resolveTreeNav([], null, "down")).toBeNull();
    expect(resolveTreeNav([], "a", "up")).toBeNull();
  });

  it("selects first when nothing selected and moving down", () => {
    expect(resolveTreeNav(ids, null, "down")).toBe("a");
  });

  it("selects last when nothing selected and moving up", () => {
    expect(resolveTreeNav(ids, null, "up")).toBe("d");
  });

  it("moves down from current selection", () => {
    expect(resolveTreeNav(ids, "a", "down")).toBe("b");
    expect(resolveTreeNav(ids, "b", "down")).toBe("c");
    expect(resolveTreeNav(ids, "c", "down")).toBe("d");
  });

  it("returns null when at end and moving down", () => {
    expect(resolveTreeNav(ids, "d", "down")).toBeNull();
  });

  it("moves up from current selection", () => {
    expect(resolveTreeNav(ids, "d", "up")).toBe("c");
    expect(resolveTreeNav(ids, "c", "up")).toBe("b");
    expect(resolveTreeNav(ids, "b", "up")).toBe("a");
  });

  it("returns null when at start and moving up", () => {
    expect(resolveTreeNav(ids, "a", "up")).toBeNull();
  });

  it("first always returns the first node", () => {
    expect(resolveTreeNav(ids, null, "first")).toBe("a");
    expect(resolveTreeNav(ids, "c", "first")).toBe("a");
  });

  it("last always returns the last node", () => {
    expect(resolveTreeNav(ids, null, "last")).toBe("d");
    expect(resolveTreeNav(ids, "b", "last")).toBe("d");
  });

  it("falls back to first when selection is stale", () => {
    expect(resolveTreeNav(ids, "missing", "down")).toBe("a");
    expect(resolveTreeNav(ids, "missing", "up")).toBe("a");
  });
});
