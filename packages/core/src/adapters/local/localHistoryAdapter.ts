/**
 * IndexedDB adapter for browser-local execution history.
 *
 * Database: "bnto-local-history"
 * Object store: "executions" (keyPath: "id", index: "by_timestamp")
 * Cap: 10 entries, oldest evicted on insert.
 */

import type { LocalHistoryEntry } from "../../types/localHistory";

const DB_NAME = "bnto-local-history";
const DB_VERSION = 1;
const STORE_NAME = "executions";
const MAX_ENTRIES = 10;

// ---------------------------------------------------------------------------
// Database lifecycle
// ---------------------------------------------------------------------------

function openDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    if (typeof indexedDB === "undefined") {
      reject(new Error("IndexedDB is not available in this environment"));
      return;
    }

    const request = indexedDB.open(DB_NAME, DB_VERSION);

    request.onupgradeneeded = () => {
      const db = request.result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        const store = db.createObjectStore(STORE_NAME, { keyPath: "id" });
        store.createIndex("by_timestamp", "timestamp", { unique: false });
      }
    };

    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

// ---------------------------------------------------------------------------
// CRUD operations
// ---------------------------------------------------------------------------

function evictOldest(store: IDBObjectStore, count: number): Promise<void> {
  const index = store.index("by_timestamp");
  const cursorRequest = index.openCursor();
  let deleted = 0;

  return new Promise((resolve, reject) => {
    cursorRequest.onsuccess = () => {
      const cursor = cursorRequest.result;
      if (cursor && deleted < count) {
        cursor.delete();
        deleted++;
        cursor.continue();
      } else {
        resolve();
      }
    };
    cursorRequest.onerror = () => reject(cursorRequest.error);
  });
}

/**
 * Add an entry, enforcing the 10-entry cap.
 * Oldest entries (by timestamp) are evicted in the same transaction.
 */
export async function addEntry(entry: LocalHistoryEntry): Promise<void> {
  const db = await openDatabase();

  try {
    const tx = db.transaction(STORE_NAME, "readwrite");
    const store = tx.objectStore(STORE_NAME);

    const count = await promisifyRequest<number>(store.count());
    if (count >= MAX_ENTRIES) {
      await evictOldest(store, count - MAX_ENTRIES + 1);
    }

    store.put(entry);
    await promisifyTransaction(tx);
  } finally {
    db.close();
  }
}

/** Walk a cursor and collect entries up to a limit, newest-first. */
function collectFromCursor(
  cursorRequest: IDBRequest<IDBCursorWithValue | null>,
  limit: number,
): Promise<LocalHistoryEntry[]> {
  const entries: LocalHistoryEntry[] = [];
  return new Promise((resolve, reject) => {
    cursorRequest.onsuccess = () => {
      const cursor = cursorRequest.result;
      if (cursor && entries.length < limit) {
        entries.push(cursor.value as LocalHistoryEntry);
        cursor.continue();
      } else {
        resolve(entries);
      }
    };
    cursorRequest.onerror = () => reject(cursorRequest.error);
  });
}

/** Get all entries, newest-first. Returns at most MAX_ENTRIES. */
export async function getEntries(): Promise<LocalHistoryEntry[]> {
  const db = await openDatabase();

  try {
    const tx = db.transaction(STORE_NAME, "readonly");
    const index = tx.objectStore(STORE_NAME).index("by_timestamp");
    return await collectFromCursor(index.openCursor(null, "prev"), MAX_ENTRIES);
  } finally {
    db.close();
  }
}

export async function clearEntries(): Promise<void> {
  const db = await openDatabase();

  try {
    const tx = db.transaction(STORE_NAME, "readwrite");
    const store = tx.objectStore(STORE_NAME);
    store.clear();
    await promisifyTransaction(tx);
  } finally {
    db.close();
  }
}

// ---------------------------------------------------------------------------
// IndexedDB promise helpers
// ---------------------------------------------------------------------------

function promisifyRequest<T>(request: IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

function promisifyTransaction(tx: IDBTransaction): Promise<void> {
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
    tx.onabort = () => reject(tx.error);
  });
}
