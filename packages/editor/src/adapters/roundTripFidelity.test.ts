/**
 * Round-trip fidelity tests — verify definition → graph → definition
 * produces equivalent output.
 *
 * These tests ensure that loading a recipe into the editor and exporting
 * it back produces a structurally equivalent definition. Field-by-field
 * comparison ensures no data loss through the adapter pipeline.
 *
 * Known intentional difference: positions are replaced by computed SLOTS
 * during definitionToGraph, so exported positions reflect the editor's
 * slot grid — not the original definition positions.
 */

import { describe, it, expect } from "vitest";
import { definitionToGraph } from "./definitionToGraph";
import { rfNodesToDefinition } from "./rfNodesToDefinition";
import { createBlankDefinition, addNode, getRecipeBySlug, RECIPES } from "@bnto/core";
import type { Definition } from "@bnto/core";

const compressImages = getRecipeBySlug("compress-images")!;
const cleanCsv = getRecipeBySlug("clean-csv")!;
const renameFiles = getRecipeBySlug("rename-files")!;
import type { RecipeMetadata } from "../store/types";

function metaOf(def: Definition): RecipeMetadata {
  const slug = def.name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
  return { id: def.id, name: def.name, slug: slug || "untitled", cloudId: null };
}

/** Strip positions from a definition tree for comparison (positions are intentionally lossy). */
function stripPositions(def: Definition): Omit<Definition, "position"> & { position?: unknown } {
  const { position: _, ...rest } = def;
  return {
    ...rest,
    ...(rest.nodes ? { nodes: rest.nodes.map(stripPositions) as Definition[] } : {}),
  };
}

describe("round-trip fidelity", () => {
  describe("blank definition", () => {
    it("preserves root identity fields", () => {
      const original = createBlankDefinition();
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(exported.id).toBe(original.id);
      expect(exported.name).toBe(original.name);
      expect(exported.type).toBe(original.type);
      expect(exported.version).toBe(original.version);
    });

    it("preserves root metadata", () => {
      const original = createBlankDefinition();
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(exported.metadata).toEqual(original.metadata);
    });

    it("preserves root input and output ports", () => {
      const original = createBlankDefinition();
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(exported.inputPorts).toEqual(original.inputPorts);
      expect(exported.outputPorts).toEqual(original.outputPorts);
    });

    it("preserves child node count", () => {
      const original = createBlankDefinition();
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(exported.nodes!.length).toBe(original.nodes!.length);
    });

    it("preserves I/O node fields through round-trip", () => {
      const original = createBlankDefinition();
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      for (const origChild of original.nodes!) {
        const exportedChild = exported.nodes!.find((n) => n.id === origChild.id)!;
        expect(exportedChild).toBeDefined();
        expect(exportedChild.id).toBe(origChild.id);
        expect(exportedChild.type).toBe(origChild.type);
        expect(exportedChild.name).toBe(origChild.name);
        expect(exportedChild.parameters).toEqual(origChild.parameters);
        expect(exportedChild.metadata).toEqual(origChild.metadata);
        expect(exportedChild.inputPorts).toEqual(origChild.inputPorts);
        expect(exportedChild.outputPorts).toEqual(origChild.outputPorts);
      }
    });
  });

  describe("with processing nodes", () => {
    it("preserves all domain fields for added nodes", () => {
      let def = createBlankDefinition();
      def = addNode(def, "image").definition;
      def = addNode(def, "spreadsheet").definition;
      def = addNode(def, "file-system").definition;

      const graph = definitionToGraph(def);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(def), graph.configs, def);

      expect(exported.nodes!.length).toBe(def.nodes!.length);

      for (const origChild of def.nodes!) {
        const exportedChild = exported.nodes!.find((n) => n.id === origChild.id)!;
        expect(exportedChild.id).toBe(origChild.id);
        expect(exportedChild.type).toBe(origChild.type);
        expect(exportedChild.name).toBe(origChild.name);
        expect(exportedChild.parameters).toEqual(origChild.parameters);
        expect(exportedChild.inputPorts).toEqual(origChild.inputPorts);
        expect(exportedChild.outputPorts).toEqual(origChild.outputPorts);
      }
    });

    it("preserves node order through round-trip", () => {
      let def = createBlankDefinition();
      def = addNode(def, "image").definition;
      def = addNode(def, "transform").definition;
      def = addNode(def, "file-system").definition;

      const graph = definitionToGraph(def);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(def), graph.configs, def);

      const originalIds = def.nodes!.map((n) => n.id);
      const exportedIds = exported.nodes!.map((n) => n.id);
      expect(exportedIds).toEqual(originalIds);
    });
  });

  describe("predefined recipes", () => {
    it("round-trips compressImages definition (with container children)", () => {
      const original = compressImages.definition;
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(stripPositions(exported)).toEqual(stripPositions(original));
    });

    it("round-trips cleanCsv definition", () => {
      const original = cleanCsv.definition;
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(stripPositions(exported)).toEqual(stripPositions(original));
    });

    it("round-trips renameFiles definition", () => {
      const original = renameFiles.definition;
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(stripPositions(exported)).toEqual(stripPositions(original));
    });

    it("round-trips all 6 predefined recipes", () => {
      for (const recipe of RECIPES) {
        const original = recipe.definition;
        const graph = definitionToGraph(original);
        const exported = rfNodesToDefinition(
          graph.nodes,
          metaOf(original),
          graph.configs,
          original,
        );

        expect(stripPositions(exported)).toEqual(stripPositions(original));
      }
    });
  });

  describe("container children preservation", () => {
    it("preserves nested children inside loop nodes", () => {
      const original = compressImages.definition;
      const loopNode = original.nodes!.find((n) => n.type === "loop")!;
      expect(loopNode.nodes!.length).toBeGreaterThan(0);

      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      const exportedLoop = exported.nodes!.find((n) => n.id === loopNode.id)!;
      expect(exportedLoop.nodes).toEqual(loopNode.nodes);
    });

    it("preserves edges inside container nodes", () => {
      const original = compressImages.definition;
      const loopNode = original.nodes!.find((n) => n.type === "loop")!;

      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      const exportedLoop = exported.nodes!.find((n) => n.id === loopNode.id)!;
      expect(exportedLoop.edges).toEqual(loopNode.edges);
    });

    it("preserves root-level edges", () => {
      const original = compressImages.definition;
      expect(original.edges!.length).toBeGreaterThan(0);

      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(exported.edges).toEqual(original.edges);
    });
  });

  describe("metadata and ports", () => {
    it("preserves root metadata with description", () => {
      const original = compressImages.definition;
      expect(original.metadata.description).toBeTruthy();

      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(exported.metadata).toEqual(original.metadata);
    });

    it("preserves child node input and output ports", () => {
      const original = cleanCsv.definition;
      const cleanNode = original.nodes!.find((n) => n.type === "spreadsheet")!;
      expect(cleanNode.inputPorts.length).toBeGreaterThan(0);
      expect(cleanNode.outputPorts.length).toBeGreaterThan(0);

      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      const exportedClean = exported.nodes!.find((n) => n.id === cleanNode.id)!;
      expect(exportedClean.inputPorts).toEqual(cleanNode.inputPorts);
      expect(exportedClean.outputPorts).toEqual(cleanNode.outputPorts);
    });

    it("preserves displayName in metadata.customData", () => {
      let def = createBlankDefinition();
      def = addNode(def, "image").definition;
      const imageNode = def.nodes!.find((n) => n.type === "image")!;

      // Inject displayName into metadata
      const withDisplayName: Definition = {
        ...def,
        nodes: def.nodes!.map((n) =>
          n.id === imageNode.id
            ? { ...n, metadata: { customData: { displayName: "My Compressor" } } }
            : n,
        ),
      };

      const graph = definitionToGraph(withDisplayName);
      const exported = rfNodesToDefinition(
        graph.nodes,
        metaOf(withDisplayName),
        graph.configs,
        withDisplayName,
      );

      const exportedImage = exported.nodes!.find((n) => n.id === imageNode.id)!;
      expect(exportedImage.metadata).toEqual({ customData: { displayName: "My Compressor" } });
    });
  });

  describe("position semantics", () => {
    it("uses slot-based positions (not original definition positions)", () => {
      const original = compressImages.definition;
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      // Positions come from SLOTS grid, not the original definition
      for (const exportedChild of exported.nodes!) {
        const origChild = original.nodes!.find((n) => n.id === exportedChild.id)!;
        // RF nodes get computed slot positions, so exported positions differ
        const rfNode = graph.nodes.find((n) => n.id === exportedChild.id)!;
        expect(exportedChild.position).toEqual(rfNode.position);
      }
    });

    it("root position is always (0,0)", () => {
      const original = compressImages.definition;
      const graph = definitionToGraph(original);
      const exported = rfNodesToDefinition(graph.nodes, metaOf(original), graph.configs, original);

      expect(exported.position).toEqual({ x: 0, y: 0 });
    });
  });

  describe("double round-trip stability", () => {
    it("definition → graph → definition → graph → definition is stable", () => {
      const original = compressImages.definition;

      // First round-trip
      const graph1 = definitionToGraph(original);
      const exported1 = rfNodesToDefinition(
        graph1.nodes,
        metaOf(original),
        graph1.configs,
        original,
      );

      // Second round-trip (using first export as the new source)
      const graph2 = definitionToGraph(exported1);
      const exported2 = rfNodesToDefinition(
        graph2.nodes,
        metaOf(exported1),
        graph2.configs,
        exported1,
      );

      // After the first round-trip adjusts positions, subsequent round-trips
      // should be fully stable (positions are already slot-based)
      expect(exported2).toEqual(exported1);
    });

    it("double round-trip is stable for all predefined recipes", () => {
      for (const recipe of RECIPES) {
        const original = recipe.definition;

        const graph1 = definitionToGraph(original);
        const exported1 = rfNodesToDefinition(
          graph1.nodes,
          metaOf(original),
          graph1.configs,
          original,
        );

        const graph2 = definitionToGraph(exported1);
        const exported2 = rfNodesToDefinition(
          graph2.nodes,
          metaOf(exported1),
          graph2.configs,
          exported1,
        );

        expect(exported2).toEqual(exported1);
      }
    });
  });
});
