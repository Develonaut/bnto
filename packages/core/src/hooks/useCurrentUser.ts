"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** Get the current authenticated user's data. */
export function useCurrentUser() {
  return useQuery(core.user.meQueryOptions());
}
