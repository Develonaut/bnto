import { describe, it, expect, vi, afterEach } from "vitest";
import { SHORTCUTS, SHORTCUT_MAP, isMacPlatform, getShortcutKeys } from "./shortcuts";
import type { ShortcutDef } from "./shortcuts";

describe("SHORTCUTS", () => {
  it("contains all expected shortcut ids", () => {
    const ids = SHORTCUTS.map((s) => s.id);
    expect(ids).toContain("undo");
    expect(ids).toContain("redo");
    expect(ids).toContain("delete");
    expect(ids).toContain("run");
    expect(ids).toContain("export");
    expect(ids).toContain("escape");
    expect(ids).toContain("help");
  });

  it("every shortcut has both mac and win keys", () => {
    for (const s of SHORTCUTS) {
      expect(s.mac.length).toBeGreaterThan(0);
      expect(s.win.length).toBeGreaterThan(0);
    }
  });

  it("every shortcut has a label and description", () => {
    for (const s of SHORTCUTS) {
      expect(s.label).toBeTruthy();
      expect(s.description).toBeTruthy();
    }
  });
});

describe("SHORTCUT_MAP", () => {
  it("provides lookup by id", () => {
    expect(SHORTCUT_MAP.get("undo")).toBeDefined();
    expect(SHORTCUT_MAP.get("undo")?.label).toBe("Undo");
  });

  it("returns undefined for unknown id", () => {
    expect(SHORTCUT_MAP.get("nonexistent")).toBeUndefined();
  });
});

describe("isMacPlatform", () => {
  const originalNavigator = globalThis.navigator;

  afterEach(() => {
    Object.defineProperty(globalThis, "navigator", {
      value: originalNavigator,
      writable: true,
    });
  });

  it("returns true for macOS platform", () => {
    Object.defineProperty(globalThis, "navigator", {
      value: { platform: "MacIntel" },
      writable: true,
    });
    expect(isMacPlatform()).toBe(true);
  });

  it("returns false for Windows platform", () => {
    Object.defineProperty(globalThis, "navigator", {
      value: { platform: "Win32" },
      writable: true,
    });
    expect(isMacPlatform()).toBe(false);
  });

  it("returns false for Linux platform", () => {
    Object.defineProperty(globalThis, "navigator", {
      value: { platform: "Linux x86_64" },
      writable: true,
    });
    expect(isMacPlatform()).toBe(false);
  });
});

describe("getShortcutKeys", () => {
  const mockDef: ShortcutDef = {
    id: "test",
    label: "Test",
    description: "Test shortcut",
    mac: ["⌘", "T"],
    win: ["Ctrl", "T"],
  };

  afterEach(() => {
    Object.defineProperty(globalThis, "navigator", {
      value: { platform: "MacIntel" },
      writable: true,
    });
  });

  it("returns mac keys on macOS", () => {
    Object.defineProperty(globalThis, "navigator", {
      value: { platform: "MacIntel" },
      writable: true,
    });
    expect(getShortcutKeys(mockDef)).toEqual(["⌘", "T"]);
  });

  it("returns win keys on Windows", () => {
    Object.defineProperty(globalThis, "navigator", {
      value: { platform: "Win32" },
      writable: true,
    });
    expect(getShortcutKeys(mockDef)).toEqual(["Ctrl", "T"]);
  });
});
