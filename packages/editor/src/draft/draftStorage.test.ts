import { describe, it, expect, beforeEach } from "vitest";
import type { Draft } from "./draftTypes";
import { DRAFT_KEY, saveDraft, loadDraft, clearDraft } from "./draftStorage";

/** Minimal in-memory Storage mock. */
function createMockStorage(): Storage {
  const store = new Map<string, string>();
  return {
    getItem: (key: string) => store.get(key) ?? null,
    setItem: (key: string, value: string) => store.set(key, value),
    removeItem: (key: string) => store.delete(key),
    clear: () => store.clear(),
    get length() {
      return store.size;
    },
    key: (index: number) => [...store.keys()][index] ?? null,
  };
}

describe("draftStorage", () => {
  let storage: Storage;

  const draft: Draft = {
    definition: {
      id: "test",
      type: "image",
      name: "Test",
      version: "1.0",
      nodes: [],
    } as unknown as Draft["definition"],
    metadata: { id: "test", name: "Test", type: "image", version: "1.0" },
    savedAt: 1710000000000,
  };

  beforeEach(() => {
    storage = createMockStorage();
  });

  it("saves and loads a draft", () => {
    saveDraft(storage, draft);
    const loaded = loadDraft(storage);
    expect(loaded).toEqual(draft);
  });

  it("returns null when no draft exists", () => {
    expect(loadDraft(storage)).toBeNull();
  });

  it("clears a saved draft", () => {
    saveDraft(storage, draft);
    clearDraft(storage);
    expect(loadDraft(storage)).toBeNull();
  });

  it("returns null for corrupted data", () => {
    storage.setItem(DRAFT_KEY, "not valid json {{{");
    expect(loadDraft(storage)).toBeNull();
  });

  it("uses the correct storage key", () => {
    saveDraft(storage, draft);
    expect(storage.getItem(DRAFT_KEY)).toBeTruthy();
    expect(DRAFT_KEY).toBe("bnto_editor_draft");
  });
});
