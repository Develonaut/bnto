import { describe, it, expect } from "vitest";
import { toDropzoneAccept } from "./toDropzoneAccept";

describe("toDropzoneAccept", () => {
  it("returns undefined for wildcard accept", () => {
    expect(toDropzoneAccept("*/*")).toBeUndefined();
  });

  it("maps MIME types to react-dropzone format", () => {
    const result = toDropzoneAccept("image/jpeg,image/png");
    expect(result).toEqual({
      "image/jpeg": [],
      "image/png": [],
    });
  });

  it("attaches extensions to the first MIME type", () => {
    const result = toDropzoneAccept("image/jpeg,.jpg,.jpeg");
    expect(result).toEqual({
      "image/jpeg": [".jpg", ".jpeg"],
    });
  });

  it("uses octet-stream for extension-only accept", () => {
    const result = toDropzoneAccept(".csv,.tsv");
    expect(result).toEqual({
      "application/octet-stream": [".csv", ".tsv"],
    });
  });

  it("handles mixed MIME types and extensions", () => {
    const result = toDropzoneAccept("text/csv,.csv");
    expect(result).toEqual({
      "text/csv": [".csv"],
    });
  });
});
