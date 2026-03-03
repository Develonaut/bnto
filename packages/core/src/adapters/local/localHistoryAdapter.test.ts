import { describe, it, expect, beforeEach } from "vitest";
import "fake-indexeddb/auto";
import { addEntry, getEntries, clearEntries } from "./localHistoryAdapter";
import type { LocalHistoryEntry } from "../../types/localHistory";

/** Create a test entry with a specific timestamp. */
function makeEntry(
  overrides: Partial<LocalHistoryEntry> = {},
): LocalHistoryEntry {
  return {
    id: overrides.id ?? crypto.randomUUID(),
    slug: "compress-images",
    status: "completed",
    timestamp: Date.now(),
    durationMs: 500,
    inputFileCount: 1,
    outputFileCount: 1,
    ...overrides,
  };
}

describe("localHistoryAdapter", () => {
  beforeEach(async () => {
    // Clear all entries between tests for isolation.
    await clearEntries();
  });

  describe("addEntry + getEntries", () => {
    it("stores and retrieves a single entry", async () => {
      const entry = makeEntry({ id: "test-1", slug: "resize-images" });
      await addEntry(entry);

      const entries = await getEntries();
      expect(entries).toHaveLength(1);
      expect(entries[0].id).toBe("test-1");
      expect(entries[0].slug).toBe("resize-images");
    });

    it("returns entries newest-first", async () => {
      await addEntry(makeEntry({ id: "old", timestamp: 1000 }));
      await addEntry(makeEntry({ id: "new", timestamp: 3000 }));
      await addEntry(makeEntry({ id: "mid", timestamp: 2000 }));

      const entries = await getEntries();
      expect(entries.map((e) => e.id)).toEqual(["new", "mid", "old"]);
    });

    it("preserves all fields", async () => {
      const entry = makeEntry({
        id: "full",
        slug: "clean-csv",
        status: "failed",
        timestamp: 1709300000000,
        durationMs: 2500,
        inputFileCount: 5,
        outputFileCount: 0,
        error: "Invalid CSV format",
      });
      await addEntry(entry);

      const [stored] = await getEntries();
      expect(stored).toEqual(entry);
    });
  });

  describe("10-entry cap", () => {
    it("enforces cap by deleting oldest entries", async () => {
      // Add 10 entries.
      for (let i = 0; i < 10; i++) {
        await addEntry(makeEntry({ id: `entry-${i}`, timestamp: i * 1000 }));
      }

      // Verify we have 10.
      let entries = await getEntries();
      expect(entries).toHaveLength(10);

      // Add an 11th entry — oldest should be evicted.
      await addEntry(
        makeEntry({ id: "entry-new", timestamp: 99000 }),
      );

      entries = await getEntries();
      expect(entries).toHaveLength(10);
      // Newest first — entry-new should be first.
      expect(entries[0].id).toBe("entry-new");
      // Oldest (entry-0) should be gone.
      expect(entries.map((e) => e.id)).not.toContain("entry-0");
    });

    it("handles adding multiple entries past the cap", async () => {
      // Fill to 10.
      for (let i = 0; i < 10; i++) {
        await addEntry(makeEntry({ id: `e-${i}`, timestamp: i * 1000 }));
      }

      // Add 3 more — the 3 oldest should be evicted.
      await addEntry(makeEntry({ id: "new-1", timestamp: 20000 }));
      await addEntry(makeEntry({ id: "new-2", timestamp: 21000 }));
      await addEntry(makeEntry({ id: "new-3", timestamp: 22000 }));

      const entries = await getEntries();
      expect(entries).toHaveLength(10);
      expect(entries.map((e) => e.id)).not.toContain("e-0");
      expect(entries.map((e) => e.id)).not.toContain("e-1");
      expect(entries.map((e) => e.id)).not.toContain("e-2");
    });
  });

  describe("clearEntries", () => {
    it("removes all entries", async () => {
      await addEntry(makeEntry({ id: "a" }));
      await addEntry(makeEntry({ id: "b" }));
      await addEntry(makeEntry({ id: "c" }));

      await clearEntries();

      const entries = await getEntries();
      expect(entries).toHaveLength(0);
    });

    it("is safe to call when empty", async () => {
      await clearEntries();
      const entries = await getEntries();
      expect(entries).toHaveLength(0);
    });
  });
});
