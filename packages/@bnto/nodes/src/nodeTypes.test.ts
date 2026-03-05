import { describe, expect, it } from "vitest";

import {
  NODE_TYPES,
  NODE_TYPE_NAMES,
  NODE_TYPE_INFO,
} from "./nodeTypes";
import { isNodeType } from "./isNodeType";
import { getNodeTypeInfo } from "./getNodeTypeInfo";
import { getBrowserCapableTypes } from "./getBrowserCapableTypes";
import { getContainerTypes } from "./getContainerTypes";

describe("NODE_TYPES", () => {
  it("contains all 12 registered node types", () => {
    expect(Object.keys(NODE_TYPES)).toHaveLength(12);
  });

  it("maps camelCase keys to kebab-case type names", () => {
    expect(NODE_TYPES.editFields).toBe("edit-fields");
    expect(NODE_TYPES.fileSystem).toBe("file-system");
    expect(NODE_TYPES.httpRequest).toBe("http-request");
    expect(NODE_TYPES.shellCommand).toBe("shell-command");
    expect(NODE_TYPES.group).toBe("group");
    expect(NODE_TYPES.image).toBe("image");
    expect(NODE_TYPES.input).toBe("input");
    expect(NODE_TYPES.loop).toBe("loop");
    expect(NODE_TYPES.output).toBe("output");
    expect(NODE_TYPES.parallel).toBe("parallel");
    expect(NODE_TYPES.spreadsheet).toBe("spreadsheet");
    expect(NODE_TYPES.transform).toBe("transform");
  });
});

describe("NODE_TYPE_NAMES", () => {
  it("contains all 12 node type name strings", () => {
    expect(NODE_TYPE_NAMES).toHaveLength(12);
  });

  it("matches the values of NODE_TYPES", () => {
    const values = Object.values(NODE_TYPES);
    for (const name of NODE_TYPE_NAMES) {
      expect(values).toContain(name);
    }
  });
});

describe("NODE_TYPE_INFO", () => {
  it("has an entry for every node type", () => {
    for (const name of NODE_TYPE_NAMES) {
      expect(NODE_TYPE_INFO[name]).toBeDefined();
      expect(NODE_TYPE_INFO[name].name).toBe(name);
    }
  });

  it("every entry has required fields", () => {
    for (const info of Object.values(NODE_TYPE_INFO)) {
      expect(info.name).toBeTruthy();
      expect(info.label).toBeTruthy();
      expect(info.description).toBeTruthy();
      expect(info.category).toBeTruthy();
      expect(typeof info.isContainer).toBe("boolean");
      expect(typeof info.browserCapable).toBe("boolean");
    }
  });

  it("container types are group, loop, and parallel", () => {
    const containers = Object.values(NODE_TYPE_INFO).filter(
      (i) => i.isContainer,
    );
    const names = containers.map((c) => c.name).sort();
    expect(names).toEqual(["group", "loop", "parallel"]);
  });

  it("server-only types are file-system, http-request, and shell-command", () => {
    const serverOnly = Object.values(NODE_TYPE_INFO).filter(
      (i) => !i.browserCapable,
    );
    const names = serverOnly.map((s) => s.name).sort();
    expect(names).toEqual(["file-system", "http-request", "shell-command"]);
  });
});

describe("IO node types", () => {
  it("input and output are registered", () => {
    expect(NODE_TYPE_INFO["input"]).toBeDefined();
    expect(NODE_TYPE_INFO["output"]).toBeDefined();
  });

  it("input and output are in the 'io' category", () => {
    expect(NODE_TYPE_INFO["input"].category).toBe("io");
    expect(NODE_TYPE_INFO["output"].category).toBe("io");
  });

  it("input and output are browser-capable", () => {
    expect(NODE_TYPE_INFO["input"].browserCapable).toBe(true);
    expect(NODE_TYPE_INFO["output"].browserCapable).toBe(true);
  });

  it("input and output are NOT containers", () => {
    expect(NODE_TYPE_INFO["input"].isContainer).toBe(false);
    expect(NODE_TYPE_INFO["output"].isContainer).toBe(false);
  });
});

describe("isNodeType", () => {
  it("returns true for valid node type names", () => {
    expect(isNodeType("image")).toBe(true);
    expect(isNodeType("http-request")).toBe(true);
    expect(isNodeType("shell-command")).toBe(true);
    expect(isNodeType("input")).toBe(true);
    expect(isNodeType("output")).toBe(true);
  });

  it("returns false for invalid strings", () => {
    expect(isNodeType("")).toBe(false);
    expect(isNodeType("unknown")).toBe(false);
    expect(isNodeType("Image")).toBe(false);
  });
});

describe("getNodeTypeInfo", () => {
  it("returns info for a valid type", () => {
    const info = getNodeTypeInfo("image");
    expect(info).toBeDefined();
    expect(info!.name).toBe("image");
    expect(info!.label).toBe("Image");
    expect(info!.category).toBe("image");
  });

  it("returns undefined for an unknown type", () => {
    expect(getNodeTypeInfo("unknown")).toBeUndefined();
  });
});

describe("getBrowserCapableTypes", () => {
  it("returns only browser-capable types", () => {
    const types = getBrowserCapableTypes();
    for (const t of types) {
      expect(t.browserCapable).toBe(true);
    }
  });

  it("excludes server-only types", () => {
    const names = getBrowserCapableTypes().map((t) => t.name);
    expect(names).not.toContain("shell-command");
    expect(names).not.toContain("file-system");
    expect(names).not.toContain("http-request");
  });
});

describe("getContainerTypes", () => {
  it("returns only container types", () => {
    const types = getContainerTypes();
    expect(types).toHaveLength(3);
    for (const t of types) {
      expect(t.isContainer).toBe(true);
    }
  });
});
