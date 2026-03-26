import { describe, it, expect } from "vitest";
import { toPairs } from "./toPairs";
import { toRecord } from "./toRecord";

describe("toPairs", () => {
  it("converts a record to an array of pairs", () => {
    expect(toPairs({ a: "1", b: "2" })).toEqual([
      { key: "a", value: "1" },
      { key: "b", value: "2" },
    ]);
  });

  it("returns empty array for empty record", () => {
    expect(toPairs({})).toEqual([]);
  });
});

describe("toRecord", () => {
  it("converts pairs to a record", () => {
    const pairs = [
      { key: "a", value: "1" },
      { key: "b", value: "2" },
    ];
    expect(toRecord(pairs)).toEqual({ a: "1", b: "2" });
  });

  it("skips pairs with empty keys", () => {
    const pairs = [
      { key: "", value: "orphan" },
      { key: "valid", value: "data" },
    ];
    expect(toRecord(pairs)).toEqual({ valid: "data" });
  });

  it("skips pairs with whitespace-only keys", () => {
    const pairs = [
      { key: "  ", value: "orphan" },
      { key: "valid", value: "data" },
    ];
    expect(toRecord(pairs)).toEqual({ valid: "data" });
  });

  it("trims key whitespace", () => {
    const pairs = [{ key: "  name  ", value: "test" }];
    expect(toRecord(pairs)).toEqual({ name: "test" });
  });

  it("last duplicate key wins", () => {
    const pairs = [
      { key: "a", value: "first" },
      { key: "a", value: "second" },
    ];
    expect(toRecord(pairs)).toEqual({ a: "second" });
  });

  it("returns empty record for empty array", () => {
    expect(toRecord([])).toEqual({});
  });

  it("preserves empty string values", () => {
    const pairs = [{ key: "name", value: "" }];
    expect(toRecord(pairs)).toEqual({ name: "" });
  });
});
