import { describe, it, expect } from "vitest";
import { storedRecipeToListItem } from "./localRecipe";
import type { StoredRecipe } from "../types";

describe("storedRecipeToListItem", () => {
  const recipe: StoredRecipe = {
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
    metadata: { id: "recipe-1", name: "My Recipe", type: "group", version: "0.1.0" },
    savedAt: 1710000000000,
    syncedAt: null,
  };

  it("maps id from metadata", () => {
    expect(storedRecipeToListItem(recipe).id).toBe("recipe-1");
  });

  it("maps name from metadata", () => {
    expect(storedRecipeToListItem(recipe).name).toBe("My Recipe");
  });

  it("counts nodes from definition", () => {
    expect(storedRecipeToListItem(recipe).nodeCount).toBe(4);
  });

  it("maps savedAt to updatedAt", () => {
    expect(storedRecipeToListItem(recipe).updatedAt).toBe(1710000000000);
  });

  it("includes syncedAt", () => {
    expect(storedRecipeToListItem(recipe).syncedAt).toBeNull();

    const synced = { ...recipe, syncedAt: 1710000001000 };
    expect(storedRecipeToListItem(synced).syncedAt).toBe(1710000001000);
  });

  it("extracts processing node type labels (excludes I/O)", () => {
    expect(storedRecipeToListItem(recipe).nodeTypes).toEqual(["Image", "File System"]);
  });

  it("deduplicates node types", () => {
    const dupes: StoredRecipe = {
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
    expect(storedRecipeToListItem(dupes).nodeTypes).toEqual(["Image", "File System"]);
  });

  it("handles missing nodes array", () => {
    const noNodes: StoredRecipe = {
      ...recipe,
      definition: { ...recipe.definition, nodes: undefined },
    };
    expect(storedRecipeToListItem(noNodes).nodeCount).toBe(0);
    expect(storedRecipeToListItem(noNodes).nodeTypes).toEqual([]);
  });
});
