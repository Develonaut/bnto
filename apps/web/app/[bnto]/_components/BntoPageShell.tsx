"use client";

import { useState } from "react";
import { useRunQuota } from "@bnto/core";
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
 *
 * File selection, config state, and upload/execution are managed here.
 * Upload infrastructure (useUploadFiles, UploadProgress) is built and
 * ready to wire — the RunButton (Wave 4) will trigger the upload flow:
 *
 *   1. User drops files → FileDropZone
 *   2. User clicks Run → RunButton calls core.uploads.uploadFiles(files)
 *   3. Upload progress shows via UploadProgress component
 *   4. On upload complete, RunButton starts execution with sessionId
 */
export function BntoPageShell({ entry }: BntoPageShellProps) {
  const { quotaExhausted } = useRunQuota();
  const [config, setConfig] = useState<BntoConfigMap[BntoSlug]>(
    DEFAULT_CONFIGS[entry.slug as BntoSlug] ?? {},
  );

  return (
    <div className="container space-y-3 text-center">
      <h1 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
        {entry.h1}
      </h1>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {entry.description}
      </p>

      <div className="mx-auto max-w-2xl space-y-4 pt-4">
        {quotaExhausted && <UpgradePrompt slug={entry.slug} reason="quota" />}

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
