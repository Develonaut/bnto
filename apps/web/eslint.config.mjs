import { defineConfig, globalIgnores } from "eslint/config";
import nextVitals from "eslint-config-next/core-web-vitals";
import nextTs from "eslint-config-next/typescript";

const eslintConfig = defineConfig([
  ...nextVitals,
  ...nextTs,
  globalIgnores([".next/**", ".next-e2e/**", "out/**", "build/**", "e2e/**", "next-env.d.ts", "public/wasm/**", "playwright-report/**", "test-results/**"]),
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
