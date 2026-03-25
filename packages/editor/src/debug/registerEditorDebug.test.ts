/**
 * registerEditorDebug tests — verifies the debug facade:
 * window attachment, state access, domain client passthrough,
 * debug commands, cleanup, and SSR safety.
 */

import { describe, it, expect, vi, afterEach } from "vitest";
import { createEditor } from "../createEditor";
import { registerEditorDebug } from "./registerEditorDebug";
import type { EditorDebugApi } from "./editorDebugApi";

/** Type-safe accessor for the debug API on window. */
function getDebugApi(): EditorDebugApi | undefined {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (globalThis as any).__bnto__?.editor;
}

/** Create an editor and register with the correct (storeApi, instance) signature. */
function register() {
  const editor = createEditor();
  const cleanup = registerEditorDebug(editor._storeApi, editor);
  return { editor, cleanup };
}

describe("registerEditorDebug", () => {
  let cleanup: (() => void) | undefined;

  afterEach(() => {
    cleanup?.();
    cleanup = undefined;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    delete (globalThis as any).__bnto__;
  });

  // --- Registration ---

  it("attaches to globalThis.__bnto__.editor", () => {
    const reg = register();
    cleanup = reg.cleanup;
    expect(getDebugApi()).toBeDefined();
  });

  it("preserves existing __bnto__ properties", () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (globalThis as any).__bnto__ = { other: 42 };
    const reg = register();
    cleanup = reg.cleanup;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect((globalThis as any).__bnto__.other).toBe(42);
    expect(getDebugApi()).toBeDefined();
  });

  // --- State access ---

  it("getState returns a deep clone", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const api = getDebugApi()!;
    const snapshot1 = api.getState();
    const snapshot2 = api.getState();
    expect(snapshot1).toEqual(snapshot2);
    expect(snapshot1).not.toBe(snapshot2);
    snapshot1.isDirty = true;
    expect(reg.editor.getState().isDirty).toBe(false);
  });

  it("setState patches the real store", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const api = getDebugApi()!;
    api.setState({ isDirty: true });
    expect(reg.editor.getState().isDirty).toBe(true);
  });

  it("subscribe fires on store changes", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const api = getDebugApi()!;
    const listener = vi.fn();
    const unsub = api.subscribe(listener);
    reg.editor.nodes.addNode("image-compress");
    expect(listener).toHaveBeenCalled();
    unsub();
  });

  it("unsubscribe stops notifications", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const api = getDebugApi()!;
    const listener = vi.fn();
    const unsub = api.subscribe(listener);
    unsub();
    reg.editor.nodes.addNode("image-compress");
    expect(listener).not.toHaveBeenCalled();
  });

  // --- Domain clients ---

  it("exposes domain clients from the instance", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const api = getDebugApi()!;
    expect(api.nodes).toBe(reg.editor.nodes);
    expect(api.definition).toBe(reg.editor.definition);
    expect(api.execution).toBe(reg.editor.execution);
    expect(api.history).toBe(reg.editor.history);
    expect(api.panels).toBe(reg.editor.panels);
  });

  // --- Debug commands ---

  it("error() sets phase to failed with message", () => {
    const reg = register();
    cleanup = reg.cleanup;
    getDebugApi()!.error("Test error");
    expect(reg.editor.getState().executionPhase).toBe("failed");
    expect(reg.editor.getState().executionErrors).toContain("Test error");
  });

  it("complete() sets phase and all nodes to completed", () => {
    const reg = register();
    cleanup = reg.cleanup;
    reg.editor.nodes.addNode("image-compress");
    getDebugApi()!.complete();
    expect(reg.editor.getState().executionPhase).toBe("completed");
    for (const status of Object.values(reg.editor.getState().executionState)) {
      expect(status).toBe("completed");
    }
  });

  it("snapshot() returns data-only object (no functions)", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const snap = getDebugApi()!.snapshot();
    expect(Object.values(snap).some((v) => typeof v === "function")).toBe(false);
  });

  it("reset() clears execution but keeps nodes", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const api = getDebugApi()!;
    reg.editor.nodes.addNode("image-compress");
    const countBefore = reg.editor.getState().nodes.length;
    api.error("bad");
    api.reset();
    const s = reg.editor.getState();
    expect(s.executionPhase).toBe("idle");
    expect(s.executionErrors).toEqual([]);
    expect(s.nodes.length).toBe(countBefore);
  });

  it("clear() wipes everything to a blank recipe", () => {
    const reg = register();
    cleanup = reg.cleanup;
    const api = getDebugApi()!;
    reg.editor.nodes.addNode("image-compress");
    const countBefore = reg.editor.getState().nodes.length;
    api.error("bad");
    api.clear();
    const s = reg.editor.getState();
    expect(s.executionPhase).toBe("idle");
    expect(s.executionErrors).toEqual([]);
    expect(s.nodes.length).toBeLessThan(countBefore);
  });

  it("step() advances execution one stage", () => {
    const reg = register();
    cleanup = reg.cleanup;
    reg.editor.nodes.addNode("image-compress");
    const msg = getDebugApi()!.step();
    expect(typeof msg).toBe("string");
    expect(reg.editor.getState().executionPhase).toBe("running");
  });

  it("run() completes execution", async () => {
    const reg = register();
    cleanup = reg.cleanup;
    reg.editor.nodes.addNode("image-compress");
    await getDebugApi()!.run(0);
    expect(reg.editor.getState().executionPhase).toBe("completed");
  });

  // --- Cleanup ---

  it("cleanup removes editor from globalThis", () => {
    const reg = register();
    expect(getDebugApi()).toBeDefined();
    reg.cleanup();
    expect(getDebugApi()).toBeUndefined();
    cleanup = undefined;
  });

  it("cleanup preserves other __bnto__ properties", () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (globalThis as any).__bnto__ = { other: 99 };
    const reg = register();
    reg.cleanup();
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect((globalThis as any).__bnto__.other).toBe(99);
    expect(getDebugApi()).toBeUndefined();
    cleanup = undefined;
  });

  // --- SSR safety ---

  it("does not throw when globalThis is available", () => {
    expect(() => {
      const reg = register();
      cleanup = reg.cleanup;
    }).not.toThrow();
  });
});
