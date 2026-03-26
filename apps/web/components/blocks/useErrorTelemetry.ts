"use client";

import { useEffect } from "react";
import { core } from "@bnto/core";

export function useErrorTelemetry(message: string, digest: string | undefined, route: string) {
  useEffect(() => {
    core.telemetry.capture("app_error", { message, route, digest });
  }, [message, digest, route]);
}
