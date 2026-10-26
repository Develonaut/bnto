/**
 * ESLint for the repo's own `scripts/` folder (ISS-65). Workspace packages
 * lint themselves with their own configs; the root isn't a package, so its
 * scripts get the same base rules here. Run by `task check:scripts` and CI.
 */
import { baseConfig } from "./eslint.config.base.mjs";

export default [
  ...baseConfig,
  // Everything at the root except `scripts/`, so a stray `eslint .` here
  // can't lint the packages or the web app with the wrong config.
  { ignores: ["*", "!scripts/"] },
];
