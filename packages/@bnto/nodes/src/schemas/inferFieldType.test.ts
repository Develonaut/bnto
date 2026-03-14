/**
 * Tests for inferFieldType — Zod type → UI control mapping.
 *
 * Verifies the mapping table documented in inferFieldType.ts:
 *   z.enum()                → select
 *   z.boolean()             → switch
 *   z.number().min().max()  → slider (bounded)
 *   z.number()              → number (unbounded)
 *   z.string()              → text
 *   z.string() + meta.control=textarea → textarea
 *   z.array(z.string())     → tagPicker
 *   z.record(z.string())    → keyValue
 *   z.record(z.unknown())   → keyValue
 */
import { describe, expect, it } from "vitest";
import { z } from "zod";
import { inferFieldType } from "./inferFieldType";

describe("inferFieldType", () => {
  it("maps z.enum() → select", () => {
    const field = z.enum(["a", "b", "c"]);
    const info = inferFieldType(field);
    expect(info.type).toBe("enum");
    expect(info.control).toBe("select");
    expect(info.required).toBe(true);
    expect(info.enumValues).toEqual(["a", "b", "c"]);
  });

  it("maps z.boolean() → switch", () => {
    const info = inferFieldType(z.boolean());
    expect(info.type).toBe("boolean");
    expect(info.control).toBe("switch");
    expect(info.required).toBe(true);
  });

  it("maps bounded z.number().min().max() → slider", () => {
    const info = inferFieldType(z.number().min(1).max(100));
    expect(info.type).toBe("number");
    expect(info.control).toBe("slider");
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });

  it("maps unbounded z.number() → number", () => {
    const info = inferFieldType(z.number());
    expect(info.type).toBe("number");
    expect(info.control).toBe("number");
    expect(info.min).toBeUndefined();
    expect(info.max).toBeUndefined();
  });

  it("maps z.number().min() (no max) → number", () => {
    const info = inferFieldType(z.number().min(1));
    expect(info.type).toBe("number");
    expect(info.control).toBe("number");
    expect(info.min).toBe(1);
    expect(info.max).toBeUndefined();
  });

  it("maps z.string() → text", () => {
    const info = inferFieldType(z.string());
    expect(info.type).toBe("string");
    expect(info.control).toBe("text");
  });

  it("maps z.string() with meta.control=textarea → textarea", () => {
    const info = inferFieldType(z.string(), {
      label: "Notes",
      description: "...",
      control: "textarea",
    });
    expect(info.type).toBe("string");
    expect(info.control).toBe("textarea");
  });

  it("maps z.string() without textarea meta → text", () => {
    const info = inferFieldType(z.string(), { label: "Name", description: "..." });
    expect(info.type).toBe("string");
    expect(info.control).toBe("text");
  });

  it("maps z.array(z.string()) → tagPicker", () => {
    const info = inferFieldType(z.array(z.string()));
    expect(info.type).toBe("array");
    expect(info.control).toBe("tagPicker");
    expect(info.required).toBe(true);
  });

  it("maps z.array(z.string()).optional() → tagPicker (not required)", () => {
    const info = inferFieldType(z.array(z.string()).optional());
    expect(info.type).toBe("array");
    expect(info.control).toBe("tagPicker");
    expect(info.required).toBe(false);
  });

  it("maps z.record(z.string()) → keyValue", () => {
    const info = inferFieldType(z.record(z.string()));
    expect(info.type).toBe("record");
    expect(info.control).toBe("keyValue");
    expect(info.required).toBe(true);
  });

  it("maps z.record(z.string()).optional() → keyValue (not required)", () => {
    const info = inferFieldType(z.record(z.string()).optional());
    expect(info.type).toBe("record");
    expect(info.control).toBe("keyValue");
    expect(info.required).toBe(false);
  });

  it("maps z.record(z.unknown()) → keyValue", () => {
    const info = inferFieldType(z.record(z.unknown()));
    expect(info.type).toBe("record");
    expect(info.control).toBe("keyValue");
  });

  it("unwraps z.optional() wrappers and marks not required", () => {
    const info = inferFieldType(z.enum(["x", "y"]).optional());
    expect(info.type).toBe("enum");
    expect(info.control).toBe("select");
    expect(info.required).toBe(false);
    expect(info.enumValues).toEqual(["x", "y"]);
  });

  it("unwraps z.default() wrappers and marks not required", () => {
    const info = inferFieldType(z.number().min(0).max(10).default(5));
    expect(info.type).toBe("number");
    expect(info.control).toBe("slider");
    expect(info.required).toBe(false);
    expect(info.min).toBe(0);
    expect(info.max).toBe(10);
  });

  it("unwraps nested optional + default and marks not required", () => {
    const info = inferFieldType(z.boolean().optional().default(true));
    expect(info.type).toBe("boolean");
    expect(info.control).toBe("switch");
    expect(info.required).toBe(false);
  });

  it("image quality field → slider (bounded 1-100)", () => {
    const quality = z.number().min(1).max(100).optional().default(80);
    const info = inferFieldType(quality);
    expect(info.control).toBe("slider");
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });

  it("does not map z.array(z.number()) → tagPicker (only string arrays)", () => {
    const info = inferFieldType(z.array(z.number()));
    expect(info.type).toBe("string");
    expect(info.control).toBe("text");
  });
});
