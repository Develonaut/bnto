import createMDX from "@next/mdx";
import type { NextConfig } from "next";
import { resolve } from "path";

const nextConfig: NextConfig = {
  pageExtensions: ["js", "jsx", "mdx", "ts", "tsx"],
  // Standalone output for Docker (Railway). Vercel ignores this — it uses its own adapter.
  output: "standalone",
  outputFileTracingRoot: resolve(__dirname, "../../"),
  transpilePackages: ["@bnto/auth", "@bnto/core"],
  devIndicators: false,
  turbopack: {
    root: resolve(__dirname, "../../"),
    // Explicit alias for Node.js subpath imports (#components/*, etc.)
    // Turbopack on Vercel doesn't resolve the `imports` field in package.json
    // when turbopack.root points to the monorepo root. This makes it explicit.
    resolveAlias: {
      "#components/*": "./components/*",
      "#lib/*": "./lib/*",
      "#hooks/*": "./hooks/*",
      "#actions/*": "./actions/*",
      "#src/*": "./src/*",
    },
  },
  // Allow e2e tests to use a separate build directory so they don't
  // corrupt the dev server's .next cache (set via NEXT_DIST_DIR env var).
  ...(process.env.NEXT_DIST_DIR ? { distDir: process.env.NEXT_DIST_DIR } : {}),
};

const withMDX = createMDX({
  options: {
    remarkPlugins: [],
    rehypePlugins: [],
  },
});

export default withMDX(nextConfig);
