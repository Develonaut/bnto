/**
 * runPipeline action tests — validates preparePipeline converts editor
 * state into a PipelineDefinition correctly and identifies errors.
 */

import { describe, it, expect } from "vitest";
import { preparePipeline, isPipelineError } from "./runPipeline";
import type { NodeConfigs } from "../adapters/types";
import type { RecipeMetadata } from "../store/types";
import { loadRecipe } from "./loadRecipe";

const metadata: RecipeMetadata = {
  id: "test",
  name: "Test Recipe",
  type: "group",
  version: "1.0.0",
};

describe("preparePipeline", () => {
  it("returns a valid PipelineDefinition from a predefined recipe", () => {
    const loaded = loadRecipe("compress-images");
    if (!loaded || !loaded.nodes || !loaded.configs || !loaded.recipeMetadata) {
      throw new Error("loadRecipe returned incomplete state");
    }

    const result = preparePipeline({
      nodes: loaded.nodes,
      configs: loaded.configs,
      recipeMetadata: loaded.recipeMetadata,
    });

    expect(isPipelineError(result)).toBe(false);
    if (isPipelineError(result)) return;

    // compress-images has at least input + processing + output nodes
    expect(result.definition.nodes.length).toBeGreaterThanOrEqual(3);
    const types = result.definition.nodes.map((n) => n.type);
    expect(types).toContain("input");
    expect(types).toContain("output");
    // At least one processing node type (not input/output)
    expect(types.some((t) => t !== "input" && t !== "output")).toBe(true);
  });

  it("sets initial execution state with processing nodes as pending", () => {
    const loaded = loadRecipe("compress-images");
    if (!loaded || !loaded.nodes || !loaded.configs || !loaded.recipeMetadata) {
      throw new Error("loadRecipe returned incomplete state");
    }

    const result = preparePipeline({
      nodes: loaded.nodes,
      configs: loaded.configs,
      recipeMetadata: loaded.recipeMetadata,
    });

    if (isPipelineError(result)) return;

    // I/O nodes should be idle, processing nodes should be pending
    expect(result.initialExecutionState["input"]).toBe("idle");
    expect(result.initialExecutionState["output"]).toBe("idle");

    // At least one processing node should be pending
    const hasPending = Object.values(result.initialExecutionState).some((s) => s === "pending");
    expect(hasPending).toBe(true);
  });

  it("produces empty pipeline from empty nodes", () => {
    const configs: NodeConfigs = {};
    const result = preparePipeline({ nodes: [], configs, recipeMetadata: metadata });
    if (!isPipelineError(result)) {
      expect(result.definition.nodes).toHaveLength(0);
    }
  });
});

describe("isPipelineError", () => {
  it("returns true for error results", () => {
    expect(isPipelineError({ errors: ["something broke"] })).toBe(true);
  });

  it("returns false for success results", () => {
    expect(
      isPipelineError({
        definition: { nodes: [] },
        initialExecutionState: {},
      }),
    ).toBe(false);
  });
});
