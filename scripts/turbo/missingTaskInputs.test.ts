/**
 * Tests for missingTaskInputs. Uses node:test (no test runner dependency).
 * Run: pnpm exec tsx --test scripts/turbo/missingTaskInputs.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { missingTaskInputs, type Task } from "./missingTaskInputs";

const BASE = "eslint.config.base.mjs";

function repo(files: Record<string, string>) {
  return {
    read(file: string) {
      const source = files[file];
      if (source === undefined) throw new Error(`unexpected read: ${file}`);
      return source;
    },
    exists: (file: string) => file in files,
  };
}

// A lint task's entry is its ESLint config.
function lint(task: Task, files: Record<string, string>) {
  return missingTaskInputs(task, ["eslint.config.mjs"], repo(files));
}

const authConfig = {
  "packages/@bnto/auth/eslint.config.mjs": `import { baseConfig } from "../../../${BASE}";`,
  [BASE]: `import eslint from "@eslint/js";`,
};

describe("missingTaskInputs: lint", () => {
  it("names a root file the config imports that the inputs don't list", () => {
    const task = { directory: "packages/@bnto/auth", inputs: ["eslint.config.mjs"] };
    assert.deepEqual(lint(task, authConfig), [BASE]);
  });

  it("passes once the inputs list the root file", () => {
    const task = {
      directory: "packages/@bnto/auth",
      inputs: ["eslint.config.mjs", `../../../${BASE}`],
    };
    assert.deepEqual(lint(task, authConfig), []);
  });

  it("ignores npm imports and files inside the package", () => {
    const files = {
      "apps/web/eslint.config.mjs": `import next from "eslint-config-next";\nimport { x } from "./eslint/rules.mjs";`,
      "apps/web/eslint/rules.mjs": `export const x = 1;`,
    };
    assert.deepEqual(lint({ directory: "apps/web", inputs: [] }, files), []);
  });

  it("ignores imports shown in comments", () => {
    const files = {
      ...authConfig,
      [BASE]: `/**\n * Usage:\n *   import { baseConfig } from "../../${BASE}";\n */\n// import "./old.mjs";\nexport const baseConfig = [];`,
    };
    const task = { directory: "packages/@bnto/auth", inputs: [`../../../${BASE}`] };
    assert.deepEqual(lint(task, files), []);
  });

  it("finds an import after a glob string that looks like a comment start", () => {
    const files = {
      "apps/web/eslint.config.mjs": `const files = ["**/*.tsx"];\nimport { baseConfig } from "../../${BASE}";\nconst ignores = ["**/*.test.ts"];`,
      [BASE]: ``,
    };
    assert.deepEqual(lint({ directory: "apps/web", inputs: [] }, files), [BASE]);
  });

  it("finds a re-export (`export … from`)", () => {
    const files = {
      "packages/ui/eslint.config.mjs": `export { default } from "../../${BASE}";`,
      [BASE]: ``,
    };
    assert.deepEqual(lint({ directory: "packages/ui", inputs: [] }, files), [BASE]);
  });

  it("finds a dynamic import", () => {
    const files = {
      "packages/ui/eslint.config.mjs": `const { baseConfig } = await import("../../${BASE}");`,
      [BASE]: ``,
    };
    assert.deepEqual(lint({ directory: "packages/ui", inputs: [] }, files), [BASE]);
  });

  it("follows a root file's own relative imports", () => {
    const files = {
      "packages/ui/eslint.config.mjs": `import { baseConfig } from "../../${BASE}";`,
      [BASE]: `export { rules } from "./eslint/rules.mjs";`,
      "eslint/rules.mjs": `import "./side-effect.mjs";`,
      "eslint/side-effect.mjs": ``,
    };
    const task = { directory: "packages/ui", inputs: [`../../${BASE}`] };
    assert.deepEqual(lint(task, files), ["eslint/rules.mjs", "eslint/side-effect.mjs"]);
  });
});

const NODES = "packages/@bnto/nodes";
const SNAPSHOT = "engine/catalog.snapshot.json";

describe("missingTaskInputs: test and build", () => {
  it("names a root JSON file a test imports, without parsing it", () => {
    const files = {
      [`${NODES}/src/catalog.test.ts`]: `import catalog from "../../../../${SNAPSHOT}";`,
      [SNAPSHOT]: `{ "not": "javascript" `,
    };
    const task = { directory: NODES, inputs: [] };
    assert.deepEqual(missingTaskInputs(task, ["src/catalog.test.ts"], repo(files)), [SNAPSHOT]);
  });

  it("passes once the inputs list it", () => {
    const files = {
      [`${NODES}/src/catalog.test.ts`]: `import catalog from "../../../../${SNAPSHOT}";`,
      [SNAPSHOT]: `{}`,
    };
    const task = { directory: NODES, inputs: [`../../../${SNAPSHOT}`] };
    assert.deepEqual(missingTaskInputs(task, ["src/catalog.test.ts"], repo(files)), []);
  });

  it("resolves extensionless and folder imports, inside the package and out", () => {
    const files = {
      "packages/core/src/run.ts": `import { a } from "./lib";\nimport { h } from "../../../test-fixtures/helpers";`,
      "packages/core/src/lib/index.ts": `import { b } from "../../../../shared/b";`,
      "shared/b.tsx": ``,
      "test-fixtures/helpers.ts": ``,
    };
    const task = { directory: "packages/core", inputs: [] };
    assert.deepEqual(missingTaskInputs(task, ["src/run.ts"], repo(files)), [
      "test-fixtures/helpers.ts",
      "shared/b.tsx",
    ]);
  });

  it("skips imports it can't resolve, such as assets", () => {
    const files = {
      "apps/web/app/layout.tsx": `import "./globals.css";\nimport x from "../logo";`,
    };
    const task = { directory: "apps/web", inputs: [] };
    assert.deepEqual(missingTaskInputs(task, ["app/layout.tsx"], repo(files)), []);
  });

  it("names a file imported twice once", () => {
    const files = {
      "packages/core/src/a.ts": `import type { T } from "../../../shared/x";\nimport { v } from "../../../shared/x";`,
      "shared/x.ts": ``,
    };
    const task = { directory: "packages/core", inputs: [] };
    assert.deepEqual(missingTaskInputs(task, ["src/a.ts"], repo(files)), ["shared/x.ts"]);
  });

  it("resolves a `.js` specifier to its `.ts` source", () => {
    const files = {
      "packages/core/src/a.ts": `import { h } from "../../../shared/helper.js";`,
      "shared/helper.ts": ``,
    };
    const task = { directory: "packages/core", inputs: [] };
    assert.deepEqual(missingTaskInputs(task, ["src/a.ts"], repo(files)), ["shared/helper.ts"]);
  });
});
