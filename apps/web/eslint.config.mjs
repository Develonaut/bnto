import { defineConfig, globalIgnores } from "eslint/config";
import nextVitals from "eslint-config-next/core-web-vitals";
import nextTs from "eslint-config-next/typescript";
import prettierConfig from "eslint-config-prettier";
import sonarjs from "eslint-plugin-sonarjs";
import react from "eslint-plugin-react";
import unicorn from "eslint-plugin-unicorn";
import { boundariesSettings, boundariesRules } from "../../eslint.config.base.mjs";
import boundaries from "eslint-plugin-boundaries";

const eslintConfig = defineConfig([
  ...nextVitals,
  ...nextTs,
  prettierConfig,
  globalIgnores([
    ".next/**",
    ".next-e2e/**",
    "out/**",
    "build/**",
    "e2e/**",
    "next-env.d.ts",
    "public/wasm/**",
    "playwright-report/**",
    "test-results/**",
  ]),
  {
    plugins: { sonarjs, boundaries, unicorn },
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

      // --- Structural quality ---
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

      // --- React hooks ---
      "react-hooks/exhaustive-deps": "error",

      // --- Style ---
      "no-else-return": ["error", { allowElseIf: false }],

      // --- Architecture boundaries ---
      ...boundariesRules,

      // --- Import quality (using eslint-plugin-import from eslint-config-next) ---
      "import/no-cycle": ["error", { maxDepth: 3 }],
      "import/no-duplicates": "error",

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

      // --- Accessibility (beyond eslint-config-next defaults) ---
      "jsx-a11y/anchor-has-content": "warn",
      "jsx-a11y/anchor-is-valid": "warn",
      "jsx-a11y/click-events-have-key-events": "warn",
      "jsx-a11y/heading-has-content": "warn",
      "jsx-a11y/interactive-supports-focus": "warn",
      "jsx-a11y/label-has-associated-control": "warn",
      "jsx-a11y/no-autofocus": ["warn", { ignoreNonDOM: true }],
      "jsx-a11y/no-noninteractive-element-interactions": "warn",
      "jsx-a11y/no-static-element-interactions": "warn",
      "jsx-a11y/no-noninteractive-tabindex": "warn",
      "jsx-a11y/no-redundant-roles": "warn",
      "jsx-a11y/tabindex-no-positive": "warn",
    },
  },
  // --- JSX cleanliness — no inline functions in JSX props ---
  {
    files: ["**/*.tsx"],
    plugins: { react },
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
  {
    files: ["components/ui/file-upload.tsx"],
    rules: {
      "react-hooks/immutability": "off",
      "jsx-a11y/role-supports-aria-props": "off",
      "@next/next/no-img-element": "off",
    },
  },
  // Dev showcase pages are long by design (showing multiple component variants)
  {
    files: ["app/(dev)/**"],
    rules: {
      "max-lines-per-function": "off",
      "max-lines": "off",
      "react/jsx-no-bind": "off",
    },
  },
  // Test files naturally have long describe blocks
  {
    files: ["**/*.test.ts", "**/*.test.tsx", "**/__tests__/**"],
    rules: {
      "max-lines-per-function": "off",
      "max-lines": "off",
    },
  },
]);

export default eslintConfig;
