"use client";

import { useQuery } from "@tanstack/react-query";
import { runsRemainingQueryOptions } from "../adapters";

/** Get the number of workflow runs remaining for the current user. */
export function useRunsRemaining() {
  return useQuery(runsRemainingQueryOptions());
}
