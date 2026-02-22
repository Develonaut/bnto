"use client";

import { useIsAuthenticated } from "@bnto/core";
import { LandingPage } from "./LandingPage";
import { Dashboard } from "./Dashboard";

/**
 * Home page content — auth users see Dashboard, unauth users see LandingPage.
 */
export function HomeContent() {
  const isAuthenticated = useIsAuthenticated();

  if (!isAuthenticated) {
    return <LandingPage />;
  }

  return <Dashboard />;
}
