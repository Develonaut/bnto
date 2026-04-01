import { describe, expect, it } from "vitest";
import { t } from "../t";
import { useT } from "../useT";
import appStrings from "../en.json";
import nodeStrings from "../generated/nodes.json";

describe("t()", () => {
  it("resolves top-level namespace keys", () => {
    expect(t("site.title")).toBe("bnto");
    expect(t("site.trustLine")).toBe("Free, instant, runs in your browser.");
  });

  it("resolves deeply nested keys", () => {
    expect(t("hero.heading")).toBe("Pick a recipe. Drop your files.");
    expect(t("errors.timeout")).toContain("took too long");
  });

  it("returns the key itself for missing paths", () => {
    // @ts-expect-error - intentionally testing invalid key
    expect(t("nonexistent.key")).toBe("nonexistent.key");
  });

  it("returns the key for partially valid paths", () => {
    // @ts-expect-error - intentionally testing partial path
    expect(t("site.nonexistent")).toBe("site.nonexistent");
  });

  it("resolves generated node type keys", () => {
    expect(t("nodeTypes.imageCompress.label")).toBe("Compress Images");
    expect(t("params.imageCompress.quality.label")).toBe("Quality");
    expect(t("categories.image")).toBe("Image");
  });

  it("resolves every app string key without returning the key itself", () => {
    const failures: string[] = [];
    function walk(obj: Record<string, unknown>, prefix: string) {
      for (const [key, value] of Object.entries(obj)) {
        if (key === "_generatedBy") continue;
        const path = prefix ? `${prefix}.${key}` : key;
        if (typeof value === "string") {
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          const result = t(path as any);
          if (result === path) failures.push(path);
        } else if (typeof value === "object" && value !== null) {
          walk(value as Record<string, unknown>, path);
        }
      }
    }
    walk(appStrings as Record<string, unknown>, "");
    walk(nodeStrings as Record<string, unknown>, "");
    expect(failures).toEqual([]);
  });
});

describe("useT()", () => {
  it("returns the t function", () => {
    const tFn = useT();
    expect(tFn).toBe(t);
  });
});
