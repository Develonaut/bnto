import { describe, it, expect } from "vitest";
import { ICON_COMPONENTS } from "./nodeIcons";
import { getNodeIcon, NODE_TYPE_NAMES } from "@bnto/core";

describe("nodeIcons", () => {
  it("has a component for every node type icon string", () => {
    const missing: string[] = [];

    for (const typeName of NODE_TYPE_NAMES) {
      const icon = getNodeIcon(typeName);
      if (!ICON_COMPONENTS[icon]) missing.push(`${typeName} → "${icon}"`);
    }

    expect(missing, `Missing ICON_COMPONENTS entries:\n${missing.join("\n")}`).toHaveLength(0);
  });

  it("has a component for every I/O mode icon", () => {
    const modes: Record<string, string[]> = {
      input: ["file-upload", "text", "url"],
      output: ["download", "display", "preview"],
    };

    const missing: string[] = [];
    for (const [nodeType, modeList] of Object.entries(modes)) {
      for (const mode of modeList) {
        const icon = getNodeIcon(nodeType as any, { mode });
        if (!ICON_COMPONENTS[icon]) missing.push(`${nodeType}(${mode}) → "${icon}"`);
      }
    }

    expect(missing, `Missing ICON_COMPONENTS entries:\n${missing.join("\n")}`).toHaveLength(0);
  });
});
