import { baseConfig } from "../../../eslint.config.base.mjs";

export default [
  ...baseConfig,
  {
    ignores: ["convex/_generated/**"],
  },
];
