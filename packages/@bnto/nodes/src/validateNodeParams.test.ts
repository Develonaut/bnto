/**
 * Tests for validateNodeParams — Zod-based parameter validation.
 *
 * Validates that node types with schemas produce correct field-level errors
 * for invalid parameters and pass cleanly for valid ones.
 * Types without schemas (http-request, shell-command) return empty arrays.
 */

import { describe, expect, it } from "vitest";

import { validateNodeParams } from "./validateNodeParams";

describe("validateNodeParams", () => {
  // ---------- Unknown type ----------

  it("returns empty for unknown node type", () => {
    expect(validateNodeParams("banana", "n1", { foo: "bar" })).toEqual([]);
  });

  // ---------- image-compress ----------

  describe("image-compress", () => {
    it("passes with valid params", () => {
      const errors = validateNodeParams("image-compress", "n1", { quality: 80 });
      expect(errors).toHaveLength(0);
    });

    it("passes with empty params (quality defaults)", () => {
      const errors = validateNodeParams("image-compress", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("fails when quality is out of range", () => {
      const errors = validateNodeParams("image-compress", "n1", { quality: 200 });
      expect(errors.some((e) => e.field === "quality")).toBe(true);
    });

    it("fails when quality is below minimum", () => {
      const errors = validateNodeParams("image-compress", "n1", { quality: 0 });
      expect(errors.some((e) => e.field === "quality")).toBe(true);
    });
  });

  // ---------- image-resize ----------

  describe("image-resize", () => {
    it("passes with valid params", () => {
      const errors = validateNodeParams("image-resize", "n1", { width: 200, quality: 80 });
      expect(errors).toHaveLength(0);
    });

    it("passes with empty params (all optional/defaulted)", () => {
      const errors = validateNodeParams("image-resize", "n1", {});
      expect(errors).toHaveLength(0);
    });
  });

  // ---------- image-convert ----------

  describe("image-convert", () => {
    it("passes with valid format", () => {
      const errors = validateNodeParams("image-convert", "n1", { format: "webp" });
      expect(errors).toHaveLength(0);
    });

    it("passes when format is missing (defaults to jpeg)", () => {
      const errors = validateNodeParams("image-convert", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("fails when format is invalid", () => {
      const errors = validateNodeParams("image-convert", "n1", { format: "gif" });
      expect(errors.length).toBeGreaterThan(0);
    });
  });

  // ---------- http-request (no schema — returns empty) ----------

  describe("http-request", () => {
    it("returns empty (no schema for this type)", () => {
      const errors = validateNodeParams("http-request", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("returns empty even with valid-looking params", () => {
      const errors = validateNodeParams("http-request", "n1", {
        url: "https://example.com",
        method: "GET",
      });
      expect(errors).toHaveLength(0);
    });
  });

  // ---------- file-rename ----------

  describe("file-rename", () => {
    it("passes with empty params (all optional)", () => {
      const errors = validateNodeParams("file-rename", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("passes with prefix param", () => {
      const errors = validateNodeParams("file-rename", "n1", { prefix: "renamed-" });
      expect(errors).toHaveLength(0);
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

  // ---------- shell-command (no schema — returns empty) ----------

  describe("shell-command", () => {
    it("returns empty (no schema for this type)", () => {
      const errors = validateNodeParams("shell-command", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("returns empty even with valid-looking params", () => {
      const errors = validateNodeParams("shell-command", "n1", { command: "echo hello" });
      expect(errors).toHaveLength(0);
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

  // ---------- spreadsheet-clean ----------

  describe("spreadsheet-clean", () => {
    it("passes with empty params (all defaulted)", () => {
      const errors = validateNodeParams("spreadsheet-clean", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("passes with explicit boolean params", () => {
      const errors = validateNodeParams("spreadsheet-clean", "n1", {
        trimWhitespace: true,
        removeEmptyRows: false,
      });
      expect(errors).toHaveLength(0);
    });
  });

  // ---------- spreadsheet-rename ----------

  describe("spreadsheet-rename", () => {
    it("passes with empty params (columns optional)", () => {
      const errors = validateNodeParams("spreadsheet-rename", "n1", {});
      expect(errors).toHaveLength(0);
    });

    it("passes with columns mapping", () => {
      const errors = validateNodeParams("spreadsheet-rename", "n1", {
        columns: { Name: "full_name" },
      });
      expect(errors).toHaveLength(0);
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
      const errors = validateNodeParams("image-convert", "my-node", { format: "gif" });
      for (const error of errors) {
        expect(error.nodeId).toBe("my-node");
        expect(error.message).toContain("my-node");
      }
    });

    it("includes field path in error messages", () => {
      const errors = validateNodeParams("image-convert", "n1", { format: "gif" });
      const formatError = errors.find((e) => e.field === "format");
      expect(formatError).toBeDefined();
      expect(formatError!.message).toContain("format");
    });
  });
});
