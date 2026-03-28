import { describe, it, expect } from "vitest";
import { NODE_TYPE_NAMES } from "@bnto/nodes";
import { getAllNodeTypes } from "./getAllNodeTypes";
import { getBrowserNodeTypes } from "./getBrowserNodeTypes";

describe("nodeTypes", () => {
  describe("getAllNodeTypes", () => {
    it("returns all registered node types", () => {
      expect(Object.keys(getAllNodeTypes())).toHaveLength(NODE_TYPE_NAMES.length);
    });

    it("every node type has required fields", () => {
      for (const info of Object.values(getAllNodeTypes())) {
        expect(info.name).toBeTruthy();
        expect(info.label).toBeTruthy();
        expect(info.category).toBeTruthy();
        expect(typeof info.isContainer).toBe("boolean");
        expect(typeof info.browserCapable).toBe("boolean");
      }
    });
  });

  describe("getBrowserNodeTypes", () => {
    it("returns only browser-capable types", () => {
      for (const info of getBrowserNodeTypes()) {
        expect(info.browserCapable).toBe(true);
      }
    });

    it("has fewer items than all node types", () => {
      const all = Object.values(getAllNodeTypes());
      const browser = getBrowserNodeTypes();
      expect(browser.length).toBeLessThan(all.length);
    });
  });
});
