"use client";

import { useState } from "react";
import { useAnonymousSession, useRunsRemaining } from "@bnto/core";
import type { BntoEntry } from "../../../lib/bnto-registry";
import { BntoConfigPanel } from "./BntoConfigPanel";
import type { BntoConfigMap, BntoSlug } from "./configs/types";
import { DEFAULT_CONFIGS } from "./configs/types";
import { FileDropZone } from "./FileDropZone";
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
  const [config, setConfig] = useState<BntoConfigMap[BntoSlug]>(
    DEFAULT_CONFIGS[entry.slug as BntoSlug] ?? {},
  );

  const quotaExhausted = !isPending && isAnonymous && runsRemaining === 0;

  return (
    <div className="container space-y-3 text-center">
      <h1 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
        {entry.h1}
      </h1>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {entry.description}
      </p>

      <div className="mx-auto max-w-2xl space-y-4 pt-4">
        {quotaExhausted && (
          <UpgradePrompt slug={entry.slug} reason="quota" />
        )}

        {!quotaExhausted && (
          <>
            <div className="text-left">
              <BntoConfigPanel
                slug={entry.slug}
                config={config}
                onChange={setConfig}
              />
            </div>
            <FileDropZone slug={entry.slug} />
          </>
        )}
      </div>
    </div>
  );
}
