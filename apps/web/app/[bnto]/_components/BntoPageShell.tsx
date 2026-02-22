"use client";

import { useAnonymousSession, useRunsRemaining } from "@bnto/core";
import { DashedLine } from "@bnto/ui/dashed-line";
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
  const { isPending, isAnonymous, isAuthenticated } = useAnonymousSession();
  const { data: runsRemaining } = useRunsRemaining();

  const quotaExhausted = isAnonymous && runsRemaining === 0;

  return (
    <div className="flex flex-1 flex-col items-center justify-center p-6">
      <div className="w-full max-w-2xl space-y-6 text-center">
        <h1 className="font-display text-3xl font-bold tracking-tight text-foreground">
          {entry.h1}
        </h1>

        <DashedLine className="mx-auto max-w-xs" />

        {isPending ? (
          <p className="text-sm text-muted-foreground">Loading...</p>
        ) : (
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">
              {isAuthenticated
                ? "Ready to run"
                : isAnonymous
                  ? "Running as guest"
                  : "Setting up session..."}
            </p>

            {quotaExhausted && (
              <UpgradePrompt slug={entry.slug} reason="quota" />
            )}

            {/* TODO(Sprint 2): Render workflow execution UI here */}
            <div className="rounded-xl border border-border bg-card p-8">
              <p className="text-sm text-muted-foreground">
                Workflow UI coming soon -- drop files here to get started.
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
