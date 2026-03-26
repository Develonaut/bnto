import { baseConfig } from "../../eslint.config.base.mjs";

export default [
  ...baseConfig,
  // createCn.ts IS the implementation — it must import tailwind-variants
  {
    files: ["src/utils/createCn.ts"],
    rules: {
      "no-restricted-imports": "off",
    },
  },
];
