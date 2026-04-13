import createMDX from "@next/mdx";
import type { NextConfig } from "next";
import { resolve } from "node:path";

const nextConfig: NextConfig = {
  pageExtensions: ["js", "jsx", "mdx", "ts", "tsx"],
  // Standalone output for containerized deploys (Tauri, self-hosted). Vercel ignores this — it uses its own adapter.
  output: "standalone",
  outputFileTracingRoot: resolve(__dirname, "../../"),
  transpilePackages: [
    "@bnto/auth",
    "@bnto/core",
    "@bnto/editor",
    "@bnto/i18n",
    "@bnto/nodes",
    "@bnto/ui",
  ],
  devIndicators: false,
  // Allow e2e tests to use a separate build directory so they don't
  // corrupt the dev server's .next cache (set via NEXT_DIST_DIR env var).
  ...(process.env.NEXT_DIST_DIR ? { distDir: process.env.NEXT_DIST_DIR } : {}),
  // PostHog's API uses trailing slashes (e.g. /e/). Without this, Next.js
  // redirects /d/e/ → /d/e which breaks event capture.
  skipTrailingSlashRedirect: true,
  // Reverse proxy for PostHog — routes analytics requests through our domain
  // so ad blockers don't intercept them. NEXT_PUBLIC_POSTHOG_HOST should be
  // set to "/d" (not the PostHog URL) when this is active. The path "/d" is
  // deliberately short and generic — "/ingest" was being blocked by EasyPrivacy
  // and uBlock Origin filter lists.
  async rewrites() {
    return [
      {
        source: "/d/static/:path*",
        destination: "https://us-assets.i.posthog.com/static/:path*",
      },
      { source: "/d/:path*", destination: "https://us.i.posthog.com/:path*" },
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
