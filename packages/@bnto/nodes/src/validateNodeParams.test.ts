/**
 * Tests for validateNodeParams — Zod-based parameter validation.
 *
 * Validates that all 12 node types produce correct field-level errors
 * for invalid parameters and pass cleanly for valid ones.
 */

import { describe, expect, it } from "vitest";

import { validateNodeParams } from "./validateNodeParams";

describe("validateNodeParams", () => {
  // ---------- Unknown type ----------

  it("returns empty for unknown node type", () => {
    expect(validateNodeParams("banana", "n1", { foo: "bar" })).toEqual([]);
  });

  // ---------- image ----------

  describe("image", () => {
    it("passes with valid params", () => {
      const errors = validateNodeParams("image", "n1", { operation: "resize", quality: 80 });
      expect(errors).toHaveLength(0);
    });

    it("fails when operation is missing", () => {
      const errors = validateNodeParams("image", "n1", {});
      expect(errors.length).toBeGreaterThan(0);
      expect(errors.some((e) => e.field === "operation")).toBe(true);
    });

    it("fails when operation is invalid", () => {
      const errors = validateNodeParams("image", "n1", { operation: "explode" });
      expect(errors.length).toBeGreaterThan(0);
    });

    it("fails when quality is out of range", () => {
      const errors = validateNodeParams("image", "n1", { operation: "resize", quality: 200 });
      expect(errors.some((e) => e.field === "quality")).toBe(true);
    });

    it("fails when quality is below minimum", () => {
      const errors = validateNodeParams("image", "n1", { operation: "resize", quality: 0 });
      expect(errors.some((e) => e.field === "quality")).toBe(true);
    });
  });

  // ---------- http-request ----------

  describe("http-request", () => {
    it("passes with valid params", () => {
      const errors = validateNodeParams("http-request", "n1", {
        url: "https://example.com",
        method: "GET",
      });
      expect(errors).toHaveLength(0);
    });

    it("fails when url is missing", () => {
      const errors = validateNodeParams("http-request", "n1", { method: "GET" });
      expect(errors.some((e) => e.field === "url")).toBe(true);
    });

    it("fails when method is invalid", () => {
      const errors = validateNodeParams("http-request", "n1", {
        url: "https://example.com",
        method: "YEET",
      });
      expect(errors.length).toBeGreaterThan(0);
    });

    it("passes with url only (method defaults to GET)", () => {
      const errors = validateNodeParams("http-request", "n1", { url: "https://example.com" });
      expect(errors).toHaveLength(0);
    });
  });

  // ---------- file-system ----------

  describe("file-system", () => {
    it("passes with valid operation", () => {
      const errors = validateNodeParams("file-system", "n1", { operation: "read" });
      expect(errors).toHaveLength(0);
    });

    it("fails when operation is missing", () => {
      const errors = validateNodeParams("file-system", "n1", {});
      expect(errors.some((e) => e.field === "operation")).toBe(true);
    });

    it("fails when operation is invalid", () => {
      const errors = validateNodeParams("file-system", "n1", { operation: "format-c" });
      expect(errors.length).toBeGreaterThan(0);
    });
  });

  // ---------- loop ----------

  describe("loop", () => {
    it("passes with valid forEach params", () => {
      const errors = validateNodeParams("loop", "n1", { mode: "forEach", items: "{{.files}}" });
      expect(errors).toHaveLength(0);
    });

    it("passes with valid times params", () => {
      const errors = validateNodeParams("loop", "n1", { mode: "times", count: 5 });
      expect(errors).toHaveLength(0);
    });

    it("passes with valid while params", () => {
      const errors = validateNodeParams("loop", "n1", { mode: "while", condition: "x < 10" });
      expect(errors).toHaveLength(0);
    });

    it("fails when mode is missing", () => {
      const errors = validateNodeParams("loop", "n1", {});
      expect(errors.some((e) => e.field === "mode")).toBe(true);
    });

    it("fails when mode is invalid", () => {
      const errors = validateNodeParams("loop", "n1", { mode: "infinite" });
      expect(errors.length).toBeGreaterThan(0);
    });
  });

  // ---------- shell-command ----------

  describe("shell-command", () => {
    it("passes with valid command", () => {
      const errors = validateNodeParams("shell-command", "n1", { command: "echo hello" });
      expect(errors).toHaveLength(0);
    });

    it("fails when command is missing", () => {
      const errors = validateNodeParams("shell-command", "n1", {});
      expect(errors.some((e) => e.field === "command")).toBe(true);
    });

    it("fails when timeout is out of range", () => {
      const errors = validateNodeParams("shell-command", "n1", {
        command: "sleep",
        timeout: 100000,
      });
      expect(errors.some((e) => e.field === "timeout")).toBe(true);
    });
  });

  // ---------- edit-fields ----------

  describe("edit-fields", () => {
    it("passes with valid values", () => {
      const errors = validateNodeParams("edit-fields", "n1", { values: { name: "test" } });
      expect(errors).toHaveLength(0);
    });

    it("fails when values is missing", () => {
      const errors = validateNodeParams("edit-fields", "n1", {});
      expect(errors.some((e) => e.field === "values")).toBe(true);
    });
  });

  // ---------- spreadsheet ----------

  describe("spreadsheet", () => {
    it("passes with required params", () => {
      const errors = validateNodeParams("spreadsheet", "n1", {
        operation: "read",
        format: "csv",
        path: "/file.csv",
      });
      expect(errors).toHaveLength(0);
    });

    it("fails when operation is missing", () => {
      const errors = validateNodeParams("spreadsheet", "n1", {});
      expect(errors.length).toBeGreaterThanOrEqual(1);
      expect(errors.some((e) => e.message.includes("operation"))).toBe(true);
    });
  });

  // ---------- transform ----------

  describe("transform", () => {
    it("passes with empty params (all optional)", () => {
      const errors = validateNodeParams("transform", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("passes with expression", () => {
      const errors = validateNodeParams("transform", "n1", { expression: "a + b" });
      expect(errors).toHaveLength(0);
    });
  });

  // ---------- group ----------

  describe("group", () => {
    it("passes with empty params (mode has default)", () => {
      const errors = validateNodeParams("group", "n1", {});
      expect(errors).toHaveLength(0);
    });
  });

  // ---------- parallel ----------

  describe("parallel", () => {
    it("passes with valid tasks", () => {
      const errors = validateNodeParams("parallel", "n1", { tasks: [{ a: 1 }] });
      expect(errors).toHaveLength(0);
    });

    it("fails when tasks is missing", () => {
      const errors = validateNodeParams("parallel", "n1", {});
      expect(errors.some((e) => e.field === "tasks")).toBe(true);
    });
  });

  // ---------- input ----------

  describe("input", () => {
    it("passes with empty params (mode has default)", () => {
      const errors = validateNodeParams("input", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("fails with invalid mode", () => {
      const errors = validateNodeParams("input", "n1", { mode: "telepathy" });
      expect(errors.length).toBeGreaterThan(0);
    });
  });

  // ---------- output ----------

  describe("output", () => {
    it("passes with empty params (mode has default)", () => {
      const errors = validateNodeParams("output", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("fails with invalid mode", () => {
      const errors = validateNodeParams("output", "n1", { mode: "smoke-signal" });
      expect(errors.length).toBeGreaterThan(0);
    });
  });

  // ---------- Error message format ----------

  describe("error messages", () => {
    it("includes nodeId in error messages", () => {
      const errors = validateNodeParams("http-request", "my-node", {});
      for (const error of errors) {
        expect(error.nodeId).toBe("my-node");
        expect(error.message).toContain("my-node");
      }
    });

    it("includes field path in error messages", () => {
      const errors = validateNodeParams("http-request", "n1", {});
      const urlError = errors.find((e) => e.field === "url");
      expect(urlError).toBeDefined();
      expect(urlError!.message).toContain("url");
    });
  });
});
