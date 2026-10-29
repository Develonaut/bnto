import { describe, it, expect, vi, beforeEach } from "vitest";
import type * as BntoCore from "@bnto/core";
import { getRecipeBySlug } from "@bnto/registry";
import { createRecipeStepperActions } from "./recipeStepperActions";
import { createRecipeStepperStore } from "./recipeStepperStore";

const mocks = vi.hoisted(() => ({
  capture: vi.fn(),
  downloadAllResults: vi.fn(),
  definition: { nodes: [] } as unknown,
}));

// Keep the real store factory; stub the telemetry, download and registry
// calls the actions make.
vi.mock("@bnto/core", async (importOriginal) => ({
  ...(await importOriginal<typeof BntoCore>()),
  core: {
    telemetry: { capture: mocks.capture },
    executions: { downloadAllResults: mocks.downloadAllResults, downloadResult: vi.fn() },
    registry: { getRecipeBySlug: () => ({ definition: mocks.definition }) },
  },
  definitionToPipeline: () => ({}),
}));

const SLUG = "compress-images";

function result(filename: string): BntoCore.BrowserFileResult {
  return { blob: new Blob(["x"]), filename, mimeType: "image/jpeg", metadata: {} };
}

function setup(
  runResults: BntoCore.BrowserFileResult[],
  browserResults: BntoCore.BrowserFileResult[] = [],
) {
  const store = createRecipeStepperStore({});
  store.getState().setFiles([new File(["x"], "a.jpg", { type: "image/jpeg" })]);
  const instance = {
    run: vi.fn().mockResolvedValue({ status: "completed", results: runResults, durationMs: 5 }),
    reset: vi.fn(),
  } as unknown as BntoCore.ExecutionInstance;
  const refs = { getBrowserResults: () => browserResults };
  return { actions: createRecipeStepperActions(store, instance, refs, SLUG), instance };
}

function downloadedEvents() {
  return mocks.capture.mock.calls.filter(([event]) => event === "result_downloaded");
}

describe("recipeStepperActions: result_downloaded", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.definition = { nodes: [] };
  });

  it("records the automatic download after a successful run", async () => {
    const results = [result("a.jpg"), result("b.jpg")];
    setup(results).actions.run();

    await vi.waitFor(() => expect(mocks.downloadAllResults).toHaveBeenCalledWith(results, SLUG));
    expect(downloadedEvents()).toEqual([
      ["result_downloaded", { slug: SLUG, fileCount: 2, trigger: "auto" }],
    ]);
  });

  it("records the downloadAll action as manual", () => {
    const results = [result("a.jpg")];
    setup([], results).actions.downloadAll();

    expect(mocks.downloadAllResults).toHaveBeenCalledWith(results, SLUG);
    expect(downloadedEvents()).toEqual([
      ["result_downloaded", { slug: SLUG, fileCount: 1, trigger: "manual" }],
    ]);
  });
});

describe("recipeStepperActions: run with a required param empty", () => {
  beforeEach(() => vi.clearAllMocks());

  it("starts no run and records nothing (ISS-73)", async () => {
    // Watermark Images ships with its overlay blank.
    mocks.definition = getRecipeBySlug("watermark-images")!.definition;
    const { actions, instance } = setup([]);
    await actions.run();

    expect(instance.run).not.toHaveBeenCalled();
    expect(mocks.capture.mock.calls.map(([event]) => event)).not.toContain("recipe_run_started");
  });
});
