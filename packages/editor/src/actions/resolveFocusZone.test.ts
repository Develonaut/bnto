import { describe, it, expect } from "vitest";
import { resolveFocusZone } from "./resolveFocusZone";

describe("resolveFocusZone", () => {
  // --- Tree → Command transitions ---

  it("Tab in tree focuses command", () => {
    const result = resolveFocusZone("tree", {
      key: "Tab",
      metaKey: false,
      inputValue: "",
    });
    expect(result).toEqual({ target: "command" });
  });

  it("/ in tree focuses command", () => {
    const result = resolveFocusZone("tree", {
      key: "/",
      metaKey: false,
      inputValue: "",
    });
    expect(result).toEqual({ target: "command" });
  });

  it("Cmd+K in tree focuses command", () => {
    const result = resolveFocusZone("tree", {
      key: "k",
      metaKey: true,
      inputValue: "",
    });
    expect(result).toEqual({ target: "command" });
  });

  it("K without meta in tree does nothing", () => {
    const result = resolveFocusZone("tree", {
      key: "k",
      metaKey: false,
      inputValue: "",
    });
    expect(result).toBeNull();
  });

  // --- Command → Tree transitions ---

  it("Escape in command with empty input focuses tree", () => {
    const result = resolveFocusZone("command", {
      key: "Escape",
      metaKey: false,
      inputValue: "",
    });
    expect(result).toEqual({ target: "tree" });
  });

  it("Escape in command with non-empty input does nothing", () => {
    const result = resolveFocusZone("command", {
      key: "Escape",
      metaKey: false,
      inputValue: "/add",
    });
    expect(result).toBeNull();
  });

  // --- No-op cases ---

  it("ArrowDown in tree returns null (handled by tree keyboard)", () => {
    const result = resolveFocusZone("tree", {
      key: "ArrowDown",
      metaKey: false,
      inputValue: "",
    });
    expect(result).toBeNull();
  });

  it("Enter in command returns null", () => {
    const result = resolveFocusZone("command", {
      key: "Enter",
      metaKey: false,
      inputValue: "/add",
    });
    expect(result).toBeNull();
  });

  it("Tab in command returns null (browser default)", () => {
    const result = resolveFocusZone("command", {
      key: "Tab",
      metaKey: false,
      inputValue: "",
    });
    expect(result).toBeNull();
  });
});
