import { describe, it, expect } from "vitest";
import { resolveGap } from "./resolveGap";

describe("resolveGap", () => {
  it("returns empty string for undefined", () => {
    expect(resolveGap(undefined)).toBe("");
  });

  it("maps simple gap sizes to Tailwind classes", () => {
    expect(resolveGap("xs")).toBe("gap-1.5");
    expect(resolveGap("sm")).toBe("gap-2");
    expect(resolveGap("md")).toBe("gap-4");
    expect(resolveGap("lg")).toBe("gap-8");
    expect(resolveGap("xl")).toBe("gap-12");
  });

  it("handles responsive gap with all breakpoints", () => {
    const result = resolveGap({ mobile: "sm", tablet: "md", desktop: "lg" });
    expect(result).toContain("gap-2");
    expect(result).toContain("md:gap-4");
    expect(result).toContain("lg:gap-8");
  });

  it("handles responsive gap with only mobile", () => {
    const result = resolveGap({ mobile: "xl" });
    expect(result).toContain("gap-12");
    expect(result).not.toContain("md:");
    expect(result).not.toContain("lg:");
  });

  it("handles responsive gap with only tablet and desktop", () => {
    const result = resolveGap({ tablet: "sm", desktop: "xl" });
    expect(result).toContain("md:gap-2");
    expect(result).toContain("lg:gap-12");
  });
});
