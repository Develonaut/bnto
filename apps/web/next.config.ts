import type { NextConfig } from "next";
import { resolve } from "path";

const nextConfig: NextConfig = {
  output: "standalone",
  outputFileTracingRoot: resolve(__dirname, "../../"),
  transpilePackages: ["@bento/auth", "@bento/core", "@bento/ui", "@bento/editor"],
  devIndicators: false,
};

export default nextConfig;
