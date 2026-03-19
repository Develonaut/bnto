import { describe, expect, it } from "vitest";

import { createBlankDefinition } from "./createBlankDefinition";
import { addNode } from "./addNode";
import { removeNode } from "./removeNode";
import { updateNodeParams } from "./updateNodeParams";
import { moveNode } from "./moveNode";
import { definitionToRecipe } from "./definitionToRecipe";
import { isValid } from "./definitionResult";
import { validateDefinition } from "./validate";
import { compressImages } from "./recipes";

describe("round-trip: full editor lifecycle", () => {
  it("create → add → configure → move → export", () => {
    // 1. Start with a blank canvas (has input + output nodes)
    const blank = createBlankDefinition();
    expect(validateDefinition(blank)).toHaveLength(0);

    // 2. Add an image node (blank starts with 2 I/O nodes)
    const r1 = addNode(blank, "image", { x: 100, y: 100 });
    expect(isValid(r1)).toBe(true);
    expect(r1.definition.nodes).toHaveLength(3);

    // 3. Add a transform node
    const r2 = addNode(r1.definition, "transform", { x: 300, y: 100 });
    expect(isValid(r2)).toBe(true);
    expect(r2.definition.nodes).toHaveLength(4);

    // 4. Configure the image node (index 2, after input & output)
    const imageNodeId = r2.definition.nodes![2]!.id;
    const r3 = updateNodeParams(r2.definition, imageNodeId, {
      operation: "compress",
      quality: 75,
    });
    expect(r3.definition.nodes![2]!.parameters.operation).toBe("compress");
    expect(r3.definition.nodes![2]!.parameters.quality).toBe(75);

    // 5. Move the transform node (index 3)
    const transformNodeId = r3.definition.nodes![3]!.id;
    const r4 = moveNode(r3.definition, transformNodeId, { x: 500, y: 200 });
    expect(r4.definition.nodes![3]!.position).toEqual({ x: 500, y: 200 });

    // 6. Export as recipe
    const recipe = definitionToRecipe(r4.definition, {
      name: "My Image Pipeline",
      category: "image",
      accept: {
        mimeTypes: ["image/jpeg", "image/png"],
        extensions: [".jpg", ".png"],
        label: "JPEG or PNG images",
      },
    });

    expect(recipe.slug).toBe("my-image-pipeline");
    expect(recipe.definition.nodes).toHaveLength(4);
    expect(recipe.definition.nodes![2]!.parameters.quality).toBe(75);
  });

  it("create → add → remove → verify cleanup", () => {
    const blank = createBlankDefinition();

    // Add three nodes (blank starts with 2 I/O nodes)
    // Use types that produce valid defaults (no required string fields)
    const r1 = addNode(blank, "image", { x: 100, y: 0 });
    const r2 = addNode(r1.definition, "transform", { x: 200, y: 0 });
    const r3 = addNode(r2.definition, "group", { x: 300, y: 0 });
    expect(r3.definition.nodes).toHaveLength(5); // 2 I/O + 3 added

    // Add edges connecting the 3 added nodes (indices 2, 3, 4)
    const node1 = r3.definition.nodes![2]!.id;
    const node2 = r3.definition.nodes![3]!.id;
    const node3 = r3.definition.nodes![4]!.id;

    const withEdges = {
      ...r3.definition,
      edges: [
        { id: "e1", source: node1, target: node2 },
        { id: "e2", source: node2, target: node3 },
      ],
    };

    // Remove the middle node — both edges should be cleaned up
    const result = removeNode(withEdges, node2);
    expect(result.definition.nodes).toHaveLength(4); // 2 I/O + 2 remaining
    // e1 referenced node2 as target, e2 referenced node2 as source — both gone
    expect(result.definition.edges).toHaveLength(0);
    expect(isValid(result)).toBe(true);
  });

  it("predefined recipe round-trip: load → modify → export", () => {
    // Load a predefined recipe and modify it
    const recipeDef = compressImages.definition;

    // Verify the predefined recipe is valid
    expect(validateDefinition(recipeDef)).toHaveLength(0);

    // Add a new node to the existing recipe
    const result = addNode(recipeDef, "edit-fields", { x: 500, y: 100 });
    expect(result.definition.nodes!.length).toBe(recipeDef.nodes!.length + 1);

    // Re-export as a custom recipe
    const custom = definitionToRecipe(result.definition, {
      name: "Custom Compress Images",
      description: "Modified compress images with extra fields",
    });

    expect(custom.slug).toBe("custom-compress-images");
    expect(custom.definition.nodes!.length).toBe(recipeDef.nodes!.length + 1);
  });

  it("all operations maintain immutability", () => {
    const original = createBlankDefinition();
    const originalJson = JSON.stringify(original);

    // Perform various operations (blank has 2 I/O nodes, image is at index 2)
    const r1 = addNode(original, "image");
    const nodeId = r1.definition.nodes![2]!.id;
    updateNodeParams(r1.definition, nodeId, { quality: 50 });
    moveNode(r1.definition, nodeId, { x: 999, y: 999 });
    removeNode(r1.definition, nodeId);

    // Original should be completely unchanged
    expect(JSON.stringify(original)).toBe(originalJson);
  });
});
