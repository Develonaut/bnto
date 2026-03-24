import { describe, it, expect } from "vitest";
import { flattenTreeEntries } from "./flattenTreeEntries";
import type { NodeListEntry } from "./buildNodeListTree";

function makeEntry(id: string, children: NodeListEntry[] = []): NodeListEntry {
  return {
    node: { id, data: {} } as NodeListEntry["node"],
    config: { nodeType: "test", name: id, parameters: {} },
    children,
    isContainer: children.length > 0,
  };
}

describe("flattenTreeEntries", () => {
  it("returns empty array for empty tree", () => {
    expect(flattenTreeEntries([], new Set())).toEqual([]);
  });

  it("returns flat list for leaf-only tree", () => {
    const entries = [makeEntry("a"), makeEntry("b"), makeEntry("c")];
    expect(flattenTreeEntries(entries, new Set())).toEqual(["a", "b", "c"]);
  });

  it("includes only the container when collapsed", () => {
    const group = makeEntry("group", [makeEntry("child1"), makeEntry("child2")]);
    const entries = [makeEntry("a"), group, makeEntry("b")];
    expect(flattenTreeEntries(entries, new Set())).toEqual(["a", "group", "b"]);
  });

  it("includes container + children when expanded", () => {
    const group = makeEntry("group", [makeEntry("child1"), makeEntry("child2")]);
    const entries = [makeEntry("a"), group, makeEntry("b")];
    const expanded = new Set(["group"]);
    expect(flattenTreeEntries(entries, expanded)).toEqual(["a", "group", "child1", "child2", "b"]);
  });

  it("handles nested containers with mixed expansion", () => {
    const inner = makeEntry("inner", [makeEntry("deep1"), makeEntry("deep2")]);
    const outer = makeEntry("outer", [makeEntry("sibling"), inner]);
    const entries = [outer];

    // outer expanded, inner collapsed
    expect(flattenTreeEntries(entries, new Set(["outer"]))).toEqual(["outer", "sibling", "inner"]);

    // both expanded
    expect(flattenTreeEntries(entries, new Set(["outer", "inner"]))).toEqual([
      "outer",
      "sibling",
      "inner",
      "deep1",
      "deep2",
    ]);
  });
});
