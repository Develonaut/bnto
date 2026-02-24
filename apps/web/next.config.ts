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
  // turbopack.root omitted: setting it to the monorepo root breaks Node.js
  // subpath imports (#components/*, etc.) on Vercel because Turbopack resolves
  // package.json `imports` relative to root, not the app directory.
  // transpilePackages handles monorepo package resolution without it.
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
