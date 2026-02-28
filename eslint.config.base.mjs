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

export const baseConfig = tseslint.config(
  eslint.configs.recommended,
  tseslint.configs.recommended,
  {
    rules: {
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
    ignores: [
      "dist/**",
      "node_modules/**",
      "**/*.test.ts",
      "**/*.test.tsx",
      "**/__tests__/**",
    ],
  },
);
