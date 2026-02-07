import type { NextConfig } from "next";
import { resolve } from "path";

const nextConfig: NextConfig = {
  output: "standalone",
  outputFileTracingRoot: resolve(__dirname, "../../"),
  transpilePackages: ["@bnto/auth", "@bnto/core", "@bnto/ui", "@bnto/editor"],
  devIndicators: false,
};

export default nextConfig;
