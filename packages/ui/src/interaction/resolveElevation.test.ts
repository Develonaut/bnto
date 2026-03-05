import { describe, it, expect } from "vitest";

import { resolveElevationClass, stripSizeElevation } from "./resolveElevation";

describe("resolveElevationClass", () => {
  it("returns undefined for true (use built-in)", () => {
    expect(resolveElevationClass(true)).toBeUndefined();
  });

  it("returns 'elevation-none' for false", () => {
    expect(resolveElevationClass(false)).toBe("elevation-none");
  });

  it("returns 'elevation-none' for 'none'", () => {
    expect(resolveElevationClass("none")).toBe("elevation-none");
  });

  it("returns 'elevation-sm' for 'sm'", () => {
    expect(resolveElevationClass("sm")).toBe("elevation-sm");
  });

  it("returns 'elevation-md' for 'md'", () => {
    expect(resolveElevationClass("md")).toBe("elevation-md");
  });

  it("returns 'elevation-lg' for 'lg'", () => {
    expect(resolveElevationClass("lg")).toBe("elevation-lg");
  });
});

describe("stripSizeElevation", () => {
  it("removes elevation-sm from a class string", () => {
    expect(stripSizeElevation("foo elevation-sm bar")).toBe("foo  bar");
  });

  it("removes elevation-md from a class string", () => {
    expect(stripSizeElevation("elevation-md other")).toBe("other");
  });

  it("removes elevation-lg from a class string", () => {
    expect(stripSizeElevation("prefix elevation-lg")).toBe("prefix");
  });

  it("removes multiple elevation tokens", () => {
    expect(stripSizeElevation("elevation-sm elevation-lg")).toBe("");
  });

  it("does not remove elevation-none", () => {
    expect(stripSizeElevation("elevation-none")).toBe("elevation-none");
  });

  it("returns empty string when only elevation token present", () => {
    expect(stripSizeElevation("elevation-md")).toBe("");
  });

  it("does not remove elevation-xl or other non-standard tokens", () => {
    expect(stripSizeElevation("elevation-xl")).toBe("elevation-xl");
  });
});
