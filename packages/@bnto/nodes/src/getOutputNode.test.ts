import { describe, expect, it } from "vitest";

import { getOutputNode } from "./getOutputNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { RECIPES } from "./recipes";
import type { Definition } from "./definition";

describe("getOutputNode", () => {
  it("finds the output node in a blank definition", () => {
    const def = createBlankDefinition();
    const outputNode = getOutputNode(def);
    expect(outputNode).toBeDefined();
    expect(outputNode!.type).toBe("output");
    expect(outputNode!.id).toBe("output");
  });

  it("finds the output node in every predefined recipe", () => {
    for (const recipe of RECIPES) {
      const outputNode = getOutputNode(recipe.definition);
      expect(outputNode).toBeDefined();
      expect(outputNode!.type).toBe("output");
      expect(outputNode!.id).toBe("output");
    }
  });

  it("returns undefined when no output node exists", () => {
    const def: Definition = {
      id: "empty",
      type: "group",
      version: "1.0.0",
      name: "Empty",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [
        {
          id: "img",
          type: "image",
          version: "1.0.0",
          name: "Image",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {},
          inputPorts: [],
          outputPorts: [],
        },
      ],
      edges: [],
    };
    expect(getOutputNode(def)).toBeUndefined();
  });

  it("returns undefined when nodes array is undefined", () => {
    const def: Definition = {
      id: "no-nodes",
      type: "group",
      version: "1.0.0",
      name: "No Nodes",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      edges: [],
    };
    expect(getOutputNode(def)).toBeUndefined();
  });
});
