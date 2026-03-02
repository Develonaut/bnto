import { describe, expect, it } from "vitest";

import { isValid } from "./definitionResult";
import type { DefinitionResult } from "./definitionResult";
import { createBlankDefinition } from "./createBlankDefinition";

describe("isValid", () => {
  it("returns true when errors array is empty", () => {
    const result: DefinitionResult = {
      definition: createBlankDefinition(),
      errors: [],
    };
    expect(isValid(result)).toBe(true);
  });

  it("returns false when errors exist", () => {
    const result: DefinitionResult = {
      definition: createBlankDefinition(),
      errors: [{ nodeId: "test", field: "type", message: "bad type" }],
    };
    expect(isValid(result)).toBe(false);
  });
});
