import { describe, it, expect } from "vitest";
import { recipeToListItem } from "./recipe";
import type { Recipe } from "../types";

describe("recipeToListItem", () => {
  const recipe: Recipe = {
    id: "recipe-1",
    name: "My Recipe",
    type: "group",
    version: "0.1.0",
    definition: {
      id: "root",
      type: "group",
      version: "0.1.0",
      name: "My Recipe",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [
        {
          id: "1",
          type: "input",
          version: "0.1.0",
          name: "Input",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {},
          inputPorts: [],
          outputPorts: [],
        },
        {
          id: "2",
          type: "image",
          version: "0.1.0",
          name: "Image",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {},
          inputPorts: [],
          outputPorts: [],
        },
        {
          id: "3",
          type: "file-system",
          version: "0.1.0",
          name: "File System",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {},
          inputPorts: [],
          outputPorts: [],
        },
        {
          id: "4",
          type: "output",
          version: "0.1.0",
          name: "Output",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {},
          inputPorts: [],
          outputPorts: [],
        },
      ],
    },
    savedAt: 1710000000000,
    syncedAt: null,
  };

  it("maps id from metadata", () => {
    expect(recipeToListItem(recipe).id).toBe("recipe-1");
  });

  it("maps name from metadata", () => {
    expect(recipeToListItem(recipe).name).toBe("My Recipe");
  });

  it("counts nodes from definition", () => {
    expect(recipeToListItem(recipe).nodeCount).toBe(4);
  });

  it("maps savedAt to updatedAt", () => {
    expect(recipeToListItem(recipe).updatedAt).toBe(1710000000000);
  });

  it("includes syncedAt", () => {
    expect(recipeToListItem(recipe).syncedAt).toBeNull();

    const synced = { ...recipe, syncedAt: 1710000001000 };
    expect(recipeToListItem(synced).syncedAt).toBe(1710000001000);
  });

  it("extracts processing node type labels (excludes I/O)", () => {
    expect(recipeToListItem(recipe).nodeTypes).toEqual(["Image", "File System"]);
  });

  it("deduplicates node types", () => {
    const dupes: Recipe = {
      ...recipe,
      definition: {
        ...recipe.definition,
        nodes: [
          {
            id: "1",
            type: "image",
            version: "0.1.0",
            name: "A",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {},
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "2",
            type: "image",
            version: "0.1.0",
            name: "B",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {},
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "3",
            type: "file-system",
            version: "0.1.0",
            name: "C",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {},
            inputPorts: [],
            outputPorts: [],
          },
        ],
      },
    };
    expect(recipeToListItem(dupes).nodeTypes).toEqual(["Image", "File System"]);
  });

  it("handles missing nodes array", () => {
    const noNodes: Recipe = {
      ...recipe,
      definition: { ...recipe.definition, nodes: undefined },
    };
    expect(recipeToListItem(noNodes).nodeCount).toBe(0);
    expect(recipeToListItem(noNodes).nodeTypes).toEqual([]);
  });
});
