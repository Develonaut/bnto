"use client";

import { api } from "@bnto/backend/convex/_generated/api";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import { getConvexClient } from "../../client";
import type { DownloadUrlsResult } from "../../types/download";

// ---------------------------------------------------------------------------
// Actions (imperative — no React hooks)
// ---------------------------------------------------------------------------

/** Generate presigned GET URLs for downloading execution output files. */
export function generateDownloadUrls(
  executionId: string,
): Promise<DownloadUrlsResult> {
  return getConvexClient().action(api.downloads.generateDownloadUrls, {
    executionId: executionId as Id<"executions">,
  });
}
