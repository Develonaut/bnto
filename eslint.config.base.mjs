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

export const baseConfig = tseslint.config(
  eslint.configs.recommended,
  tseslint.configs.recommended,
  prettierConfig,
  {
    plugins: { sonarjs },
    rules: {
      // --- Complexity & size enforcement (code-standards.md) ---

      // Cyclomatic complexity — flag functions with too many paths
      complexity: ["error", { max: 10 }],

      // Cognitive complexity (sonarjs) — readability metric
      "sonarjs/cognitive-complexity": ["error", 15],

      // Function length — 20-line target, 30 hard cap (skip blanks/comments)
      "max-lines-per-function": [
        "error",
        { max: 30, skipBlankLines: true, skipComments: true, IIFEs: true },
      ],

      // File length — 250-line hard cap from code-standards.md
      "max-lines": ["error", { max: 250, skipBlankLines: true, skipComments: true }],

      // --- Structural quality (cherry-picked sonarjs rules) ---
      "sonarjs/no-identical-conditions": "error",
      "sonarjs/no-collapsible-if": "error",
      "sonarjs/no-duplicated-branches": "error",

      // --- TypeScript rules ---

      // Align with code-standards.md: no `any` without justification
      "@typescript-eslint/no-explicit-any": "warn",

      // Allow unused vars prefixed with _ (common pattern for destructuring)
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],

      // Allow empty functions (common in tests and default callbacks)
      "@typescript-eslint/no-empty-function": "off",

      // Allow non-null assertions sparingly (Convex patterns use them)
      "@typescript-eslint/no-non-null-assertion": "warn",
    },
  },
  {
    ignores: ["dist/**", "node_modules/**", "**/*.test.ts", "**/*.test.tsx", "**/__tests__/**"],
  },
);
