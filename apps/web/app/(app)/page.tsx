"use client";

import dynamic from "next/dynamic";

const HomeContent = dynamic(
  () => import("./_components/HomeContent").then((m) => m.HomeContent),
  { ssr: false },
);

/**
 * Home page — / route.
 *
 * Uses ssr: false because HomeContent needs Convex hooks (useIsAuthenticated).
 */
export default function HomePage() {
  return <HomeContent />;
}
