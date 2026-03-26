/**
 * Shared ESLint flat config for TypeScript library packages.
 *
 * Usage — in any package's `eslint.config.mjs`:
 *
 *   import { baseConfig } from "../../eslint.config.base.mjs";
 *   export default [...baseConfig];
 *
 * The web app (`apps/web/`) uses its own config extending `eslint-config-next`
 * instead of this base. This config is for non-Next.js packages only.
 */
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import prettierConfig from "eslint-config-prettier";
import sonarjs from "eslint-plugin-sonarjs";
import react from "eslint-plugin-react";
import boundaries from "eslint-plugin-boundaries";
import importX from "eslint-plugin-import-x";
import unicorn from "eslint-plugin-unicorn";
import checkFile from "eslint-plugin-check-file";

/** Architecture boundary config — shared with apps/web/eslint.config.mjs */
export const boundariesSettings = {
  "boundaries/elements": [
    { type: "app", pattern: ["apps/*"], mode: "folder" },
    { type: "editor", pattern: ["packages/editor"], mode: "folder" },
    { type: "form", pattern: ["packages/@bnto/form"], mode: "folder" },
    { type: "core", pattern: ["packages/core"], mode: "folder" },
    { type: "registry", pattern: ["packages/@bnto/registry"], mode: "folder" },
    { type: "nodes", pattern: ["packages/@bnto/nodes"], mode: "folder" },
    { type: "ui", pattern: ["packages/ui"], mode: "folder" },
    { type: "auth", pattern: ["packages/@bnto/auth"], mode: "folder" },
    { type: "backend", pattern: ["packages/@bnto/backend"], mode: "folder" },
  ],
};

export const boundariesRules = {
  "boundaries/element-types": [
    "error",
    {
      default: "disallow",
      rules: [
        { from: "app", allow: ["core", "form", "registry", "ui", "editor", "auth"] },
        { from: "editor", allow: ["core", "form", "ui"] },
        { from: "form", allow: ["core", "ui"] },
        { from: "core", allow: ["registry", "auth", "backend"] },
        { from: "registry", allow: ["nodes"] },
      ],
    },
  ],
};

export const baseConfig = tseslint.config(
  eslint.configs.recommended,
  tseslint.configs.recommended,
  prettierConfig,
  {
    plugins: { sonarjs, react, boundaries, "import-x": importX, unicorn },
    settings: { ...boundariesSettings },
    rules: {
      // --- Complexity & size enforcement (code-standards.md) ---
      complexity: ["error", { max: 10 }],
      "sonarjs/cognitive-complexity": ["error", 15],
      "max-lines-per-function": [
        "error",
        { max: 30, skipBlankLines: true, skipComments: true, IIFEs: true },
      ],
      "max-lines": ["error", { max: 250, skipBlankLines: true, skipComments: true }],

      // --- Structural quality (sonarjs) ---
      "sonarjs/no-identical-conditions": "error",
      "sonarjs/no-collapsible-if": "error",
      "sonarjs/no-duplicated-branches": "error",

      // --- TypeScript rules ---
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-inferrable-types": "error",
      "@typescript-eslint/consistent-type-imports": [
        "error",
        { prefer: "type-imports", fixStyle: "inline-type-imports" },
      ],
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],
      "@typescript-eslint/no-empty-function": "off",
      "@typescript-eslint/no-non-null-assertion": "warn",

      // --- Style ---
      "no-else-return": ["error", { allowElseIf: false }],

      // --- Architecture boundaries ---
      ...boundariesRules,

      // --- Import quality ---
      "import-x/no-cycle": ["error", { maxDepth: 3 }],
      "import-x/no-duplicates": "error",

      // --- Code quality (unicorn cherry-picks) ---
      "unicorn/no-abusive-eslint-disable": "error",
      "unicorn/prefer-node-protocol": "error",
      "unicorn/no-useless-undefined": "error",
      "unicorn/consistent-function-scoping": "warn",

      // --- Restricted imports ---
      "no-restricted-imports": [
        "error",
        {
          paths: [
            {
              name: "tailwind-variants",
              message: "Use createCn() from @bnto/ui instead.",
            },
          ],
        },
      ],
    },
  },
  // JSX cleanliness — no inline arrow functions or .bind() in JSX props
  {
    files: ["**/*.tsx"],
    rules: {
      "react/jsx-no-bind": [
        "error",
        {
          ignoreDOMComponents: false,
          ignoreRefs: true,
          allowArrowFunctions: false,
          allowFunctions: false,
          allowBind: false,
        },
      ],
    },
  },
  // File naming: PascalCase for .tsx (packages only — Next.js excluded)
  {
    files: ["**/*.tsx"],
    ignores: ["**/index.tsx"],
    plugins: { "check-file": checkFile },
    rules: {
      "check-file/filename-naming-convention": [
        "error",
        { "**/*.tsx": "PASCAL_CASE" },
        { ignoreMiddleExtensions: true },
      ],
    },
  },
  // File naming: camelCase or PascalCase for .ts (no snake_case or kebab-case)
  {
    files: ["**/*.ts"],
    ignores: ["**/index.ts", "**/*.test.ts", "**/*.config.ts", "**/*.d.ts", "**/generated/**"],
    plugins: { "check-file": checkFile },
    rules: {
      "check-file/filename-naming-convention": [
        "error",
        { "**/*.ts": "+([A-Za-z])*([A-Za-z0-9])" },
        { ignoreMiddleExtensions: true },
      ],
    },
  },
  {
    ignores: ["dist/**", "node_modules/**", "**/*.test.ts", "**/*.test.tsx", "**/__tests__/**"],
  },
);
