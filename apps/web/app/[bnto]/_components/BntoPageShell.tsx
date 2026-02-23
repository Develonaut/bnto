"use client";

import { useAnonymousSession, useRunsRemaining } from "@bnto/core";
import type { BntoEntry } from "../../../lib/bnto-registry";
import { UpgradePrompt } from "./UpgradePrompt";

interface BntoPageShellProps {
  entry: BntoEntry;
}

/**
 * Client shell for bnto tool pages.
 *
 * Lazily creates an anonymous session so users can run workflows
 * without signing up. The workflow UI renders immediately -- the
 * session arriving is async and non-blocking.
 */
export function BntoPageShell({ entry }: BntoPageShellProps) {
  const { isPending, isAnonymous } = useAnonymousSession();
  const { data: runsRemaining } = useRunsRemaining();

  const quotaExhausted = isAnonymous && runsRemaining === 0;

  return (
    <div className="container space-y-3 text-center">
      <h1 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
        {entry.h1}
      </h1>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {entry.description}
      </p>

      {isPending ? (
        <p className="text-sm text-muted-foreground pt-4">Loading...</p>
      ) : (
        <div className="mx-auto max-w-2xl space-y-4 pt-4">
          {quotaExhausted && (
            <UpgradePrompt slug={entry.slug} reason="quota" />
          )}

          {/* TODO(Sprint 2): Render workflow execution UI here */}
          <div className="rounded-xl border border-border bg-card p-8">
            <p className="text-sm text-muted-foreground">
              Drop files here to get started.
            </p>
          </div>
        </div>
      )}
    </div>
  );
}
