import { describe, it, expect, beforeEach } from "vitest";
import type { Draft } from "./draftTypes";
import {
  DRAFT_KEY_PREFIX,
  draftKey,
  saveDraft,
  loadDraft,
  clearDraft,
  listAllDrafts,
} from "./draftStorage";

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
    metadata: { id: "recipe-1", name: "Test", type: "image", version: "1.0", cloudId: null },
    savedAt: 1710000000000,
    syncedAt: null,
  };

  const draft2: Draft = {
    definition: {
      id: "test2",
      type: "image",
      name: "Second",
      version: "1.0",
      nodes: [],
    } as unknown as Draft["definition"],
    metadata: { id: "recipe-2", name: "Second", type: "image", version: "1.0", cloudId: null },
    savedAt: 1710000001000,
    syncedAt: null,
  };

  beforeEach(() => {
    storage = createMockStorage();
  });

  it("saves and loads a draft by recipe id", () => {
    saveDraft(storage, draft);
    const loaded = loadDraft(storage, "recipe-1");
    expect(loaded).toEqual(draft);
  });

  it("loads the latest draft when no recipeId is provided", () => {
    saveDraft(storage, draft);
    const loaded = loadDraft(storage);
    expect(loaded).toEqual(draft);
  });

  it("returns null when no draft exists", () => {
    expect(loadDraft(storage)).toBeNull();
  });

  it("clears a draft by recipe id", () => {
    saveDraft(storage, draft);
    clearDraft(storage, "recipe-1");
    expect(loadDraft(storage, "recipe-1")).toBeNull();
  });

  it("clears the latest draft when no recipeId is provided", () => {
    saveDraft(storage, draft);
    clearDraft(storage);
    expect(loadDraft(storage)).toBeNull();
  });

  it("isolates per-recipe drafts", () => {
    saveDraft(storage, draft);
    saveDraft(storage, draft2);

    expect(loadDraft(storage, "recipe-1")).toEqual(draft);
    expect(loadDraft(storage, "recipe-2")).toEqual(draft2);
  });

  it("latest points to the most recently saved draft", () => {
    saveDraft(storage, draft);
    saveDraft(storage, draft2);

    const latest = loadDraft(storage);
    expect(latest?.metadata.id).toBe("recipe-2");
  });

  it("returns null for corrupted data", () => {
    storage.setItem(draftKey("recipe-1"), "not valid json {{{");
    expect(loadDraft(storage, "recipe-1")).toBeNull();
  });

  it("uses per-recipe key format", () => {
    saveDraft(storage, draft);
    expect(storage.getItem(`${DRAFT_KEY_PREFIX}:recipe-1`)).toBeTruthy();
    expect(storage.getItem(`${DRAFT_KEY_PREFIX}:_latest`)).toBe("recipe-1");
  });

  it("draftKey builds the correct key", () => {
    expect(draftKey("abc")).toBe("bnto_editor_draft:abc");
  });

  it("DRAFT_KEY_PREFIX is the expected value", () => {
    expect(DRAFT_KEY_PREFIX).toBe("bnto_editor_draft");
  });

  describe("listAllDrafts", () => {
    it("returns empty array when no drafts exist", () => {
      expect(listAllDrafts(storage)).toEqual([]);
    });

    it("returns all saved drafts", () => {
      saveDraft(storage, draft);
      saveDraft(storage, draft2);
      const all = listAllDrafts(storage);
      expect(all).toHaveLength(2);
    });

    it("sorts newest-first by savedAt", () => {
      saveDraft(storage, draft);
      saveDraft(storage, draft2);
      const all = listAllDrafts(storage);
      expect(all[0].metadata.id).toBe("recipe-2");
      expect(all[1].metadata.id).toBe("recipe-1");
    });

    it("skips the _latest index key", () => {
      saveDraft(storage, draft);
      // _latest is set by saveDraft — should not appear as a draft
      const all = listAllDrafts(storage);
      expect(all).toHaveLength(1);
      expect(all[0].metadata.id).toBe("recipe-1");
    });

    it("skips corrupted entries", () => {
      saveDraft(storage, draft);
      storage.setItem(`${DRAFT_KEY_PREFIX}:bad`, "not valid json {{{");
      const all = listAllDrafts(storage);
      expect(all).toHaveLength(1);
      expect(all[0].metadata.id).toBe("recipe-1");
    });

    it("ignores non-draft keys in storage", () => {
      saveDraft(storage, draft);
      storage.setItem("some_other_key", "value");
      storage.setItem("another:key", "value");
      const all = listAllDrafts(storage);
      expect(all).toHaveLength(1);
    });

    it("reflects removals after clearing a draft", () => {
      saveDraft(storage, draft);
      saveDraft(storage, draft2);
      clearDraft(storage, "recipe-1");
      const all = listAllDrafts(storage);
      expect(all).toHaveLength(1);
      expect(all[0].metadata.id).toBe("recipe-2");
    });
  });
});
