import { baseConfig } from "../../eslint.config.base.mjs";

export default [
  ...baseConfig,
  {
    ignores: ["src/__tests__/integration/**"],
  },
];
