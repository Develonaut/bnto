import { describe, it, expect } from "vitest";
import { resolveDelete, canTriggerRun, resolveEscape } from "./shortcutActions";
import type { PanelState } from "../store/types";

describe("resolveDelete", () => {
  it("returns the selected node ID for a regular node", () => {
    expect(resolveDelete("node-1", "image")).toBe("node-1");
  });

  it("returns null when no node is selected", () => {
    expect(resolveDelete(null, null)).toBeNull();
  });

  it("returns null for input I/O node (handler-level guard)", () => {
    expect(resolveDelete("node-io", "input")).toBeNull();
  });

  it("returns null for output I/O node (handler-level guard)", () => {
    expect(resolveDelete("node-io", "output")).toBeNull();
  });

  it("returns node ID when nodeType is null (unknown type)", () => {
    expect(resolveDelete("node-1", null)).toBe("node-1");
  });
});

describe("canTriggerRun", () => {
  it("returns true when idle with files", () => {
    expect(canTriggerRun("idle", 3)).toBe(true);
  });

  it("returns true when completed with files (re-run)", () => {
    expect(canTriggerRun("completed", 2)).toBe(true);
  });

  it("returns true when failed with files (retry)", () => {
    expect(canTriggerRun("failed", 1)).toBe(true);
  });

  it("returns false when already running", () => {
    expect(canTriggerRun("running", 5)).toBe(false);
  });

  it("returns false when no files are loaded", () => {
    expect(canTriggerRun("idle", 0)).toBe(false);
  });

  it("returns false when running with no files", () => {
    expect(canTriggerRun("running", 0)).toBe(false);
  });
});

describe("resolveEscape", () => {
  const closedPanels: PanelState = {
    config: false,
    palette: false,
    run: false,
    help: false,
    save: false,
  };
  const openConfig: PanelState = {
    config: true,
    palette: false,
    run: false,
    help: false,
    save: false,
  };
  const openRun: PanelState = {
    config: false,
    palette: false,
    run: true,
    help: false,
    save: false,
  };
  const bothOpen: PanelState = {
    config: true,
    palette: false,
    run: true,
    help: false,
    save: false,
  };

  it("closes config panel when open", () => {
    const result = resolveEscape(null, openConfig);
    expect(result.closeConfig).toBe(true);
    expect(result.closeRun).toBe(false);
    expect(result.deselect).toBe(false);
  });

  it("closes run panel when open", () => {
    const result = resolveEscape(null, openRun);
    expect(result.closeConfig).toBe(false);
    expect(result.closeRun).toBe(true);
    expect(result.deselect).toBe(false);
  });

  it("closes both panels and deselects when all active", () => {
    const result = resolveEscape("node-1", bothOpen);
    expect(result.closeConfig).toBe(true);
    expect(result.closeRun).toBe(true);
    expect(result.deselect).toBe(true);
  });

  it("does nothing when no panels open and no selection", () => {
    const result = resolveEscape(null, closedPanels);
    expect(result.closeConfig).toBe(false);
    expect(result.closeRun).toBe(false);
    expect(result.deselect).toBe(false);
  });

  it("only deselects when panels are closed but node is selected", () => {
    const result = resolveEscape("node-2", closedPanels);
    expect(result.closeConfig).toBe(false);
    expect(result.closeRun).toBe(false);
    expect(result.deselect).toBe(true);
  });
});
