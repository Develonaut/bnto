import { describe, it, expect } from "vitest";
import { stashFilesForTransfer, claimTransferredFiles } from "./fileTransfer";

function fakeFile(name: string): File {
  return new File(["data"], name, { type: "image/png" });
}

describe("fileTransfer", () => {
  it("stash then claim returns files and clears the slot", () => {
    const files = [fakeFile("a.png"), fakeFile("b.png")];
    stashFilesForTransfer("r1", files);

    const claimed = claimTransferredFiles("r1");
    expect(claimed).toEqual(files);

    // Second claim returns empty — one-shot
    expect(claimTransferredFiles("r1")).toEqual([]);
  });

  it("claim without stash returns empty array", () => {
    expect(claimTransferredFiles("nonexistent")).toEqual([]);
  });

  it("stash with empty array is a no-op", () => {
    stashFilesForTransfer("r2", []);
    expect(claimTransferredFiles("r2")).toEqual([]);
  });

  it("multiple recipe IDs are independent", () => {
    const filesA = [fakeFile("a.png")];
    const filesB = [fakeFile("b.png")];
    stashFilesForTransfer("rA", filesA);
    stashFilesForTransfer("rB", filesB);

    expect(claimTransferredFiles("rA")).toEqual(filesA);
    expect(claimTransferredFiles("rB")).toEqual(filesB);
  });
});
