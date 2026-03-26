import { defineConfig, globalIgnores } from "eslint/config";
import nextVitals from "eslint-config-next/core-web-vitals";
import nextTs from "eslint-config-next/typescript";
import prettierConfig from "eslint-config-prettier";
import sonarjs from "eslint-plugin-sonarjs";
import react from "eslint-plugin-react";

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
    plugins: { sonarjs, react },
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

      // --- JSX cleanliness — no inline functions in JSX props ---
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
