import type { NextConfig } from "next";
import { resolve } from "path";

const nextConfig: NextConfig = {
  // Standalone output for Docker (Railway). Vercel ignores this — it uses its own adapter.
  output: "standalone",
  outputFileTracingRoot: resolve(__dirname, "../../"),
  transpilePackages: ["@bnto/auth", "@bnto/core", "@bnto/ui", "@bnto/editor"],
  devIndicators: false,
  turbopack: {
    root: resolve(__dirname, "../../"),
  },
};

export default nextConfig;
