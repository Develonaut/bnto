import createMDX from "@next/mdx";
import type { NextConfig } from "next";
import { resolve } from "path";

const nextConfig: NextConfig = {
  pageExtensions: ["js", "jsx", "mdx", "ts", "tsx"],
  // Standalone output for Docker (Railway). Vercel ignores this — it uses its own adapter.
  output: "standalone",
  outputFileTracingRoot: resolve(__dirname, "../../"),
  transpilePackages: ["@bnto/auth", "@bnto/core", "@bnto/ui"],
  devIndicators: false,
  // Allow e2e tests to use a separate build directory so they don't
  // corrupt the dev server's .next cache (set via NEXT_DIST_DIR env var).
  ...(process.env.NEXT_DIST_DIR ? { distDir: process.env.NEXT_DIST_DIR } : {}),
  // Reverse proxy for PostHog — routes analytics requests through our domain
  // so ad blockers don't intercept them. NEXT_PUBLIC_POSTHOG_HOST should be
  // set to "/ingest" (not the PostHog URL) when this is active.
  async rewrites() {
    return [
      { source: "/ingest/static/:path*", destination: "https://us-assets.i.posthog.com/static/:path*" },
      { source: "/ingest/:path*", destination: "https://us.i.posthog.com/:path*" },
    ];
  },
};

const withMDX = createMDX({
  options: {
    remarkPlugins: [],
    rehypePlugins: [],
  },
});

export default withMDX(nextConfig);
