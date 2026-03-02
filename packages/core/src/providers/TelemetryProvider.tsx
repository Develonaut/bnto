"use client";

import { useEffect, useRef } from "react";
import { core } from "../core";

interface TelemetryProviderProps {
  /** PostHog project API key. If missing, telemetry is silently disabled. */
  apiKey?: string;
  /** PostHog ingest host (e.g. https://us.i.posthog.com). */
  host?: string;
  /**
   * Current URL pathname — passed from the app layer's router hook.
   * Triggers a $pageview capture on change. Kept as a prop so @bnto/core
   * stays transport-agnostic (no next/navigation import).
   */
  pathname?: string;
  children: React.ReactNode;
}

/**
 * Initializes product telemetry (PostHog) and tracks SPA page views.
 *
 * Config and pathname are injected from the app layer — this keeps
 * @bnto/core transport-agnostic (no NEXT_PUBLIC_* env vars, no
 * framework-specific router hooks inside core).
 *
 * Renders children immediately — telemetry is a fire-and-forget
 * side effect, never blocks rendering.
 */
export function TelemetryProvider({ apiKey, host, pathname, children }: TelemetryProviderProps) {
  const initializedRef = useRef(false);

  // Initialize PostHog once
  useEffect(() => {
    if (!apiKey || !host) return;
    core.telemetry.init({ apiKey, host });
    initializedRef.current = true;
  }, [apiKey, host]);

  // Track page views on pathname changes
  useEffect(() => {
    if (!pathname) return;
    core.telemetry.capture("$pageview", { $current_url: pathname });
  }, [pathname]);

  return <>{children}</>;
}
