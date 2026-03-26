import { defineConfig, globalIgnores } from "eslint/config";
import nextVitals from "eslint-config-next/core-web-vitals";
import nextTs from "eslint-config-next/typescript";
import prettierConfig from "eslint-config-prettier";
import sonarjs from "eslint-plugin-sonarjs";

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
    plugins: { sonarjs },
    rules: {
      // --- Complexity & size enforcement (code-standards.md) ---
      complexity: ["warn", { max: 10 }],
      "sonarjs/cognitive-complexity": ["warn", 15],
      "max-lines-per-function": [
        "warn",
        { max: 30, skipBlankLines: true, skipComments: true, IIFEs: true },
      ],
      "max-lines": ["warn", { max: 250, skipBlankLines: true, skipComments: true }],

      // --- Structural quality (cherry-picked sonarjs rules) ---
      "sonarjs/no-identical-conditions": "error",
      "sonarjs/no-collapsible-if": "warn",
      "sonarjs/no-duplicated-branches": "warn",
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
]);

export default eslintConfig;
