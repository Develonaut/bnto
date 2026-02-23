"use client";

import { useAnonymousSession } from "./useAnonymousSession";
import { useRunsRemaining } from "./useRunsRemaining";

/**
 * Encapsulates run quota state for the current session.
 *
 * Combines anonymous session status with runs remaining to derive
 * whether the user has exhausted their free quota.
 */
export function useRunQuota() {
  const { isPending, isAnonymous } = useAnonymousSession();
  const { data: runsRemaining } = useRunsRemaining();

  const quotaExhausted = !isPending && isAnonymous && runsRemaining === 0;

  return {
    isPending,
    isAnonymous,
    runsRemaining: runsRemaining ?? null,
    quotaExhausted,
  };
}
