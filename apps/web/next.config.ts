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
  },
};

const withMDX = createMDX({
  options: {
    remarkPlugins: [],
    rehypePlugins: [],
  },
});

export default withMDX(nextConfig);
