import { describe, expect, it } from "vitest";
import { compressImages, optimizeImagesForWeb } from "@bnto/registry";
import { getNodeSchema, getNodeParamFields, getVisibleParams } from "@bnto/core";
import { extractProcessingNodes } from "../_utils/extractProcessingNodes";

/**
 * Logic-level tests for DynamicRecipeConfig.
 *
 * Validates that the component's data pipeline works: extracting nodes,
 * looking up schemas, and resolving visible params. Render tests are
 * deferred to E2E (no @testing-library/react in apps/web).
 */

describe("DynamicRecipeConfig data pipeline", () => {
  it("single-node recipe produces one schema form config", () => {
    const nodes = extractProcessingNodes(compressImages.definition);
    expect(nodes).toHaveLength(1);

    const schema = getNodeSchema(nodes[0].type);
    expect(schema).toBeDefined();
    expect(schema!.params).toHaveProperty("quality");
  });

  it("multi-node recipe produces schemas for each processing node", () => {
    const nodes = extractProcessingNodes(optimizeImagesForWeb.definition);
    expect(nodes).toHaveLength(3);

    for (const node of nodes) {
      const schema = getNodeSchema(node.type);
      expect(schema).toBeDefined();
    }
  });

  it("field configs exist for each processing node type", () => {
    const nodes = extractProcessingNodes(optimizeImagesForWeb.definition);
    for (const node of nodes) {
      const fields = getNodeParamFields(node.type);
      expect(fields).toBeDefined();
    }
  });

  it("visible params resolve correctly with node parameters", () => {
    const nodes = extractProcessingNodes(compressImages.definition);
    const node = nodes[0];
    const visible = getVisibleParams(node.type, node.parameters);
    expect(visible.length).toBeGreaterThan(0);
    expect(visible).toContain("quality");
  });

  it("multi-node visible params resolve independently per node", () => {
    const nodes = extractProcessingNodes(optimizeImagesForWeb.definition);
    const resizeVisible = getVisibleParams(nodes[0].type, nodes[0].parameters);
    const convertVisible = getVisibleParams(nodes[1].type, nodes[1].parameters);
    const compressVisible = getVisibleParams(nodes[2].type, nodes[2].parameters);

    expect(resizeVisible).toContain("width");
    expect(convertVisible).toContain("format");
    expect(compressVisible).toContain("quality");
  });
});
