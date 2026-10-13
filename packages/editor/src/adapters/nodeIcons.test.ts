/**
 * nodeIcons tests — every icon name the node system can emit resolves to a component.
 */

import { describe, it, expect } from "vitest";
import { NODE_TYPE_NAMES, NODE_PARAM_FIELDS, getNodeIcon } from "@bnto/core";
import { ICON_COMPONENTS } from "./nodeIcons";

/** Every icon getNodeIcon can return: per type with no params, plus per mode. */
function allEmittedIcons(): Set<string> {
  const icons = new Set<string>();
  for (const nodeType of NODE_TYPE_NAMES) {
    icons.add(getNodeIcon(nodeType));
    const modes = NODE_PARAM_FIELDS[nodeType]?.mode?.options ?? [];
    for (const { value } of modes) icons.add(getNodeIcon(nodeType, { mode: value }));
  }
  return icons;
}

describe("ICON_COMPONENTS", () => {
  it("maps every icon name getNodeIcon can emit", () => {
    const unmapped = [...allEmittedIcons()].filter((icon) => !ICON_COMPONENTS[icon]);
    expect(unmapped).toEqual([]);
  });

  it("covers the input/output mode icons", () => {
    expect(getNodeIcon("input", { mode: "none" })).toBe("circle-off");
    expect(getNodeIcon("output", { mode: "overwrite" })).toBe("replace");
    expect(getNodeIcon("output", { mode: "message" })).toBe("message-square");
  });
});
