import { describe, it, expect } from "vitest";
import { createZipBlob } from "./createZipBlob";
import { unzipSync } from "fflate";
import type { BrowserFileResult } from "../../types/wasm";

function makeResult(
  filename: string,
  content: string,
  mimeType = "text/plain",
): BrowserFileResult {
  return {
    blob: new Blob([content], { type: mimeType }),
    filename,
    mimeType,
    metadata: {},
  };
}

describe("createZipBlob", () => {
  it("creates a valid ZIP blob from a single file", async () => {
    const results = [makeResult("hello.txt", "Hello, world!")];
    const zipBlob = await createZipBlob(results);

    expect(zipBlob.type).toBe("application/zip");
    expect(zipBlob.size).toBeGreaterThan(0);

    // Verify contents with fflate's unzip
    const buffer = new Uint8Array(await zipBlob.arrayBuffer());
    const entries = unzipSync(buffer);

    expect(Object.keys(entries)).toEqual(["hello.txt"]);
    expect(new TextDecoder().decode(entries["hello.txt"])).toBe(
      "Hello, world!",
    );
  });

  it("creates a ZIP with multiple files", async () => {
    const results = [
      makeResult("a.txt", "Alpha"),
      makeResult("b.csv", "name,age\nAlice,30"),
      makeResult("c.jpg", "\xff\xd8\xff", "image/jpeg"),
    ];
    const zipBlob = await createZipBlob(results);
    const entries = unzipSync(new Uint8Array(await zipBlob.arrayBuffer()));

    expect(Object.keys(entries)).toHaveLength(3);
    expect(new TextDecoder().decode(entries["a.txt"])).toBe("Alpha");
    expect(new TextDecoder().decode(entries["b.csv"])).toBe(
      "name,age\nAlice,30",
    );
  });

  it("deduplicates identical filenames", async () => {
    const results = [
      makeResult("photo.jpg", "first"),
      makeResult("photo.jpg", "second"),
      makeResult("photo.jpg", "third"),
    ];
    const zipBlob = await createZipBlob(results);
    const entries = unzipSync(new Uint8Array(await zipBlob.arrayBuffer()));

    const names = Object.keys(entries).sort();
    expect(names).toEqual(["photo (2).jpg", "photo (3).jpg", "photo.jpg"]);

    expect(new TextDecoder().decode(entries["photo.jpg"])).toBe("first");
    expect(new TextDecoder().decode(entries["photo (2).jpg"])).toBe("second");
    expect(new TextDecoder().decode(entries["photo (3).jpg"])).toBe("third");
  });

  it("handles files without extensions in deduplication", async () => {
    const results = [
      makeResult("README", "one"),
      makeResult("README", "two"),
    ];
    const zipBlob = await createZipBlob(results);
    const entries = unzipSync(new Uint8Array(await zipBlob.arrayBuffer()));

    expect(Object.keys(entries).sort()).toEqual(["README", "README (2)"]);
  });

  it("preserves binary content exactly", async () => {
    const binaryData = new Uint8Array([0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10]);
    const blob = new Blob([binaryData], { type: "image/jpeg" });
    const results: BrowserFileResult[] = [
      { blob, filename: "image.jpg", mimeType: "image/jpeg", metadata: {} },
    ];

    const zipBlob = await createZipBlob(results);
    const entries = unzipSync(new Uint8Array(await zipBlob.arrayBuffer()));

    expect(entries["image.jpg"]).toEqual(binaryData);
  });

  it("handles empty results array", async () => {
    const zipBlob = await createZipBlob([]);

    expect(zipBlob.type).toBe("application/zip");
    expect(zipBlob.size).toBeGreaterThan(0);

    const entries = unzipSync(new Uint8Array(await zipBlob.arrayBuffer()));
    expect(Object.keys(entries)).toHaveLength(0);
  });
});
