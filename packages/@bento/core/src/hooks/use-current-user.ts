"use client";

import { useQuery } from "@tanstack/react-query";
import { currentUserQueryOptions } from "../adapters";

/** Get the current authenticated user's data. */
export function useCurrentUser() {
  return useQuery(currentUserQueryOptions());
}
