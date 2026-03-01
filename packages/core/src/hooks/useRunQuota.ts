"use client";

import { useAnonymousSession } from "./useAnonymousSession";
import { useServerQuota } from "./useServerQuota";

/**
 * Encapsulates run quota state for the current session.
 *
 * Combines anonymous session status with server-node quota to derive
 * whether the user has exhausted their server-node allowance.
 * Browser executions are unlimited and not gated here.
 */
export function useRunQuota() {
  const { isPending, isAnonymous } = useAnonymousSession();
  const { data: quota } = useServerQuota();

  const quotaExhausted =
    !isPending && isAnonymous && quota?.serverRunsRemaining === 0;

  return {
    isPending,
    isAnonymous,
    serverRunsRemaining: quota?.serverRunsRemaining ?? null,
    quotaExhausted,
  };
}
