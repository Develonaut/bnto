import { describe, expect, it } from "vitest";

import { getInputNode } from "./getInputNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { RECIPES } from "./recipes";
import type { Definition } from "./definition";

describe("getInputNode", () => {
  it("finds the input node in a blank definition", () => {
    const def = createBlankDefinition();
    const inputNode = getInputNode(def);
    expect(inputNode).toBeDefined();
    expect(inputNode!.type).toBe("input");
    expect(inputNode!.id).toBe("input");
  });

  it("finds the input node in every predefined recipe", () => {
    for (const recipe of RECIPES) {
      const inputNode = getInputNode(recipe.definition);
      expect(inputNode).toBeDefined();
      expect(inputNode!.type).toBe("input");
      expect(inputNode!.id).toBe("input");
    }
  });

  it("returns undefined when no input node exists", () => {
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
    expect(getInputNode(def)).toBeUndefined();
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
    expect(getInputNode(def)).toBeUndefined();
  });
});
