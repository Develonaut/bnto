/**
 * InputRenderer tests — the input node's mode picks the widget.
 */

import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import type { Definition } from "@bnto/core";
import { CURRENT_FORMAT_VERSION } from "@bnto/core";
import { InputRenderer } from "./InputRenderer";

function nodeDef(id: string, type: string, parameters: Definition["parameters"]): Definition {
  return {
    id,
    type,
    version: CURRENT_FORMAT_VERSION,
    name: id,
    position: { x: 0, y: 0 },
    metadata: {},
    parameters,
    inputPorts: [],
    outputPorts: [],
  };
}

function renderWithMode(mode: string): string {
  const definition = {
    ...nodeDef("recipe", "group", {}),
    nodes: [nodeDef("in", "input", { mode })],
  };
  return renderToStaticMarkup(
    createElement(InputRenderer, { definition, files: [], onFilesChange: () => {} }),
  );
}

describe("InputRenderer", () => {
  it("renders a no-input state for none mode instead of a dropzone", () => {
    const html = renderWithMode("none");
    expect(html).toContain("No input needed");
    expect(html).not.toContain("Drag &amp; drop");
  });

  it("renders the file dropzone for file-upload mode", () => {
    expect(renderWithMode("file-upload")).toContain("Drag &amp; drop");
  });
});
