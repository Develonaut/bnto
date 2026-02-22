"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** Get the number of workflow runs remaining for the current user. */
export function useRunsRemaining() {
  return useQuery(core.user.runsRemainingQueryOptions());
}
