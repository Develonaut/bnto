/**
 * Tests for missingLintInputs. Uses node:test (no test runner dependency).
 * Run: pnpm exec tsx --test scripts/turbo/missingLintInputs.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { missingLintInputs } from "./missingLintInputs";

const BASE = "eslint.config.base.mjs";

function reader(files: Record<string, string>) {
  return (file: string) => {
    const source = files[file];
    if (source === undefined) throw new Error(`unexpected read: ${file}`);
    return source;
  };
}

const authConfig = {
  "packages/@bnto/auth/eslint.config.mjs": `import { baseConfig } from "../../../${BASE}";`,
  [BASE]: `import eslint from "@eslint/js";`,
};

describe("missingLintInputs", () => {
  it("names a root file the config imports that the inputs don't list", () => {
    const task = { directory: "packages/@bnto/auth", inputs: ["eslint.config.mjs"] };
    assert.deepEqual(missingLintInputs(task, reader(authConfig)), [BASE]);
  });

  it("passes once the inputs list the root file", () => {
    const task = {
      directory: "packages/@bnto/auth",
      inputs: ["eslint.config.mjs", `../../../${BASE}`],
    };
    assert.deepEqual(missingLintInputs(task, reader(authConfig)), []);
  });

  it("ignores npm imports and files inside the package", () => {
    const files = {
      "apps/web/eslint.config.mjs": `import next from "eslint-config-next";\nimport { x } from "./eslint/rules.mjs";`,
      "apps/web/eslint/rules.mjs": `export const x = 1;`,
    };
    assert.deepEqual(missingLintInputs({ directory: "apps/web", inputs: [] }, reader(files)), []);
  });

  it("ignores imports shown in comments", () => {
    const files = {
      ...authConfig,
      [BASE]: `/**\n * Usage:\n *   import { baseConfig } from "../../${BASE}";\n */\n// import "./old.mjs";\nexport const baseConfig = [];`,
    };
    const task = { directory: "packages/@bnto/auth", inputs: [`../../../${BASE}`] };
    assert.deepEqual(missingLintInputs(task, reader(files)), []);
  });

  it("follows a root file's own relative imports", () => {
    const files = {
      "packages/ui/eslint.config.mjs": `import { baseConfig } from "../../${BASE}";`,
      [BASE]: `export { rules } from "./eslint/rules.mjs";`,
      "eslint/rules.mjs": `import "./side-effect.mjs";`,
      "eslint/side-effect.mjs": ``,
    };
    const task = { directory: "packages/ui", inputs: [`../../${BASE}`] };
    assert.deepEqual(missingLintInputs(task, reader(files)), [
      "eslint/rules.mjs",
      "eslint/side-effect.mjs",
    ]);
  });
});
