import { describe, expect, it } from "vitest";

import type { Definition, Edge } from "./definition";
import { validateDefinition, validateEdges } from "./validate";
import { createBlankDefinition } from "./createBlankDefinition";
import { GENERATED_RECIPES } from "./generated/recipes";

/** Creates a minimal node; `group` type keeps parameter validation out of the way. */
function node(id: string, type = "group", overrides: Partial<Definition> = {}): Definition {
  return {
    id,
    type,
    version: "1.0.0",
    name: id,
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    ...overrides,
  };
}

function edge(id: string, source: string, target: string): Edge {
  return { id, source, target };
}

/** A root level with the given children and edges. */
function level(nodes: Definition[], edges: Edge[]): Definition {
  return node("root", "group", { nodes, edges });
}

const io = [node("input", "input"), node("output", "output")];

function messages(def: Definition): string[] {
  return validateEdges(def).map((e) => e.message);
}

describe("validateEdges — valid graphs", () => {
  it("accepts a linear chain listed out of node order", () => {
    const def = level(
      [...io, node("b"), node("a")],
      [edge("e1", "input", "a"), edge("e2", "a", "b"), edge("e3", "b", "output")],
    );
    expect(validateEdges(def)).toHaveLength(0);
  });

  it("accepts fan-out and fan-in", () => {
    const def = level(
      [...io, node("a"), node("b"), node("join")],
      [
        edge("e1", "input", "a"),
        edge("e2", "input", "b"),
        edge("e3", "a", "join"),
        edge("e4", "b", "join"),
        edge("e5", "join", "output"),
      ],
    );
    expect(validateEdges(def)).toHaveLength(0);
  });

  it("accepts a node with no incoming edge (a fresh start)", () => {
    const def = level(
      [...io, node("a"), node("fresh")],
      [edge("e1", "input", "a"), edge("e2", "fresh", "output"), edge("e3", "a", "output")],
    );
    expect(validateEdges(def)).toHaveLength(0);
  });
});

describe("validateEdges — cycles", () => {
  it("refuses a two-node cycle and names its path", () => {
    const def = level(
      [...io, node("a"), node("b")],
      [
        edge("e1", "input", "a"),
        edge("e2", "a", "b"),
        edge("e3", "b", "a"),
        edge("e4", "b", "output"),
      ],
    );
    expect(messages(def)).toEqual(["edges in group 'root' form a cycle: a → b → a"]);
  });

  it("refuses a self-loop", () => {
    const def = level(
      [...io, node("a")],
      [edge("e1", "input", "a"), edge("e2", "a", "a"), edge("e3", "a", "output")],
    );
    expect(messages(def)).toEqual(["edges in group 'root' form a cycle: a → a"]);
  });

  it("names a longer cycle starting from its first node in node order", () => {
    const def = level(
      [...io, node("a"), node("b"), node("c")],
      [
        edge("e1", "input", "a"),
        edge("e2", "c", "b"),
        edge("e3", "b", "a"),
        edge("e4", "a", "c"),
        edge("e5", "c", "output"),
      ],
    );
    expect(messages(def)).toEqual(["edges in group 'root' form a cycle: a → c → b → a"]);
  });
});

describe("validateEdges — refused edges", () => {
  it("refuses a duplicate edge (same source and target)", () => {
    const def = level(
      [...io, node("a")],
      [edge("e1", "input", "a"), edge("e2", "a", "output"), edge("e3", "a", "output")],
    );
    expect(messages(def)).toEqual(["edge 'e3' in group 'root' duplicates edge 'e2' (a → output)"]);
  });

  it("refuses an edge into the input node", () => {
    const def = level(
      [...io, node("a")],
      [edge("e1", "input", "a"), edge("e2", "a", "output"), edge("e3", "a", "input")],
    );
    expect(messages(def)).toContain(
      "edge 'e3' in group 'root' points into input node 'input' (nothing flows into an input)",
    );
  });

  it("refuses an edge out of the output node", () => {
    const def = level(
      [...io, node("a")],
      [edge("e1", "input", "a"), edge("e2", "a", "output"), edge("e3", "output", "a")],
    );
    expect(messages(def)).toContain(
      "edge 'e3' in group 'root' leaves output node 'output' (nothing flows out of an output)",
    );
  });

  it("reports errors with the existing shape (group nodeId, edges field)", () => {
    const def = level([...io, node("a")], [edge("e1", "a", "a"), edge("e2", "a", "output")]);
    for (const error of validateEdges(def)) {
      expect(error.nodeId).toBe("root");
      expect(error.field).toBe("edges");
    }
  });
});

describe("validateEdges — disconnected nodes", () => {
  it("refuses a processing node that touches no edge", () => {
    const def = level(
      [...io, node("a"), node("stray")],
      [edge("e1", "input", "a"), edge("e2", "a", "output")],
    );
    expect(messages(def)).toEqual([
      "node 'stray' in group 'root' touches no edge (connect it or remove it)",
    ]);
  });

  it("refuses an output node that touches no edge", () => {
    const def = level([...io, node("a")], [edge("e1", "input", "a")]);
    expect(messages(def)).toEqual([
      "node 'output' in group 'root' touches no edge (connect it or remove it)",
    ]);
  });

  it("does not refuse an input node that touches no edge", () => {
    const def = level([...io, node("a")], [edge("e1", "a", "output")]);
    expect(validateEdges(def)).toHaveLength(0);
  });

  it("does not apply when the level has no edges (linear fallback)", () => {
    expect(validateEdges(level([...io, node("a"), node("b")], []))).toHaveLength(0);
    const noEdges = node("root", "group", { nodes: [...io, node("a"), node("b")] });
    expect(validateEdges(noEdges)).toHaveLength(0);
  });
});

describe("validateDefinition — graph rules per level", () => {
  it("checks a container's children against the container's own edges", () => {
    const inner = node("loop-1", "group", {
      nodes: [node("x"), node("y")],
      edges: [edge("i1", "x", "y"), edge("i2", "y", "x")],
    });
    const def = level(
      [...io, inner],
      [edge("e1", "input", "loop-1"), edge("e2", "loop-1", "output")],
    );
    const errors = validateDefinition(def).filter((e) => e.field === "edges");
    expect(errors.map((e) => e.message)).toEqual([
      "edges in group 'loop-1' form a cycle: x → y → x",
    ]);
    expect(errors[0]?.nodeId).toBe("loop-1");
  });

  it("refuses a disconnected child only inside a container that has edges", () => {
    const wired = node("g1", "group", {
      nodes: [node("x"), node("y"), node("lonely")],
      edges: [edge("i1", "x", "y")],
    });
    const unwired = node("g2", "group", { nodes: [node("p"), node("q")], edges: [] });
    const def = level([...io, wired, unwired], []);
    const errors = validateDefinition(def).filter((e) => e.field === "edges");
    expect(errors.map((e) => e.message)).toEqual([
      "node 'lonely' in group 'g1' touches no edge (connect it or remove it)",
    ]);
  });

  it("does not let root edges satisfy a container's children", () => {
    const inner = node("g1", "group", {
      nodes: [node("x"), node("y")],
      edges: [edge("i1", "x", "y")],
    });
    const def = level(
      [...io, inner],
      [edge("e1", "input", "g1"), edge("e2", "g1", "output"), edge("e3", "x", "y")],
    );
    const messagesOut = validateDefinition(def).map((e) => e.message);
    expect(messagesOut).toContain(
      "edge 'e3' in group 'root' has invalid source 'x' (node doesn't exist)",
    );
  });
});

describe("validateEdges — shipped definitions stay valid", () => {
  it("createBlankDefinition() is valid", () => {
    expect(validateDefinition(createBlankDefinition())).toHaveLength(0);
  });

  it.each(GENERATED_RECIPES.map((r) => [r.slug, r.definition] as const))(
    "catalog recipe %s is valid",
    (_slug, definition) => {
      expect(validateDefinition(definition)).toHaveLength(0);
    },
  );
});
