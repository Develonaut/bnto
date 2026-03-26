import { baseConfig } from "../../../eslint.config.base.mjs";

export default [
  ...baseConfig,
  {
    ignores: ["convex/_generated/**"],
  },
  // Convex convention: snake_case filenames (no hyphens allowed by Convex)
  {
    files: ["**/*.ts"],
    rules: {
      "check-file/filename-naming-convention": "off",
    },
  },
];
