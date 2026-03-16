import { describe, it, expect, vi } from "vitest";
import type { Definition } from "@bnto/nodes";
import type { RecipeMetadata } from "../store/types";
import { serializeDraft } from "./serializeDraft";

describe("serializeDraft", () => {
  const definition = {
    id: "test",
    type: "image",
    name: "Test Recipe",
    version: "1.0",
    nodes: [],
  } as unknown as Definition;

  const metadata: RecipeMetadata = {
    id: "test",
    name: "Test Recipe",
    type: "image",
    version: "1.0",
    cloudId: null,
  };

  it("returns a Draft with definition, metadata, and timestamp", () => {
    vi.spyOn(Date, "now").mockReturnValue(1710000000000);

    const result = serializeDraft(definition, metadata);

    expect(result.definition).toBe(definition);
    expect(result.metadata).toBe(metadata);
    expect(result.savedAt).toBe(1710000000000);
    expect(result.syncedAt).toBeNull();

    vi.restoreAllMocks();
  });

  it("uses the current timestamp", () => {
    const before = Date.now();
    const result = serializeDraft(definition, metadata);
    const after = Date.now();

    expect(result.savedAt).toBeGreaterThanOrEqual(before);
    expect(result.savedAt).toBeLessThanOrEqual(after);
  });

  it("preserves syncedAt when provided", () => {
    const result = serializeDraft(definition, metadata, 1710000000000);
    expect(result.syncedAt).toBe(1710000000000);
  });

  it("defaults syncedAt to null when omitted", () => {
    const result = serializeDraft(definition, metadata);
    expect(result.syncedAt).toBeNull();
  });
});
