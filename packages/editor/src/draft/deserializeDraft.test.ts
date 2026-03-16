import { describe, it, expect } from "vitest";
import { deserializeDraft } from "./deserializeDraft";

describe("deserializeDraft", () => {
  const validDraft = {
    definition: { id: "test", type: "image", name: "Test", version: "1.0", nodes: [] },
    metadata: { id: "test", name: "Test", type: "image", version: "1.0" },
    savedAt: 1710000000000,
  };

  it("parses valid JSON into a Draft", () => {
    const result = deserializeDraft(JSON.stringify(validDraft));
    expect(result).toEqual(validDraft);
  });

  it("returns null for empty string", () => {
    expect(deserializeDraft("")).toBeNull();
  });

  it("returns null for invalid JSON", () => {
    expect(deserializeDraft("{bad json")).toBeNull();
  });

  it("returns null when definition is missing", () => {
    const { definition: _, ...rest } = validDraft;
    expect(deserializeDraft(JSON.stringify(rest))).toBeNull();
  });

  it("returns null when metadata is missing", () => {
    const { metadata: _, ...rest } = validDraft;
    expect(deserializeDraft(JSON.stringify(rest))).toBeNull();
  });

  it("returns null when savedAt is missing", () => {
    const { savedAt: _, ...rest } = validDraft;
    expect(deserializeDraft(JSON.stringify(rest))).toBeNull();
  });

  it("returns null when savedAt is not a number", () => {
    expect(deserializeDraft(JSON.stringify({ ...validDraft, savedAt: "not-a-number" }))).toBeNull();
  });

  it("returns null for JSON primitives", () => {
    expect(deserializeDraft("42")).toBeNull();
    expect(deserializeDraft('"string"')).toBeNull();
    expect(deserializeDraft("null")).toBeNull();
  });
});
