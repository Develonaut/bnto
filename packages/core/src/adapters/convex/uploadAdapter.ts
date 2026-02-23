"use client";

import { api } from "@bnto/backend/convex/_generated/api";
import { getConvexClient } from "../../client";
import type { UploadFileInput, UploadSession } from "../../types/upload";

// ---------------------------------------------------------------------------
// Mutations (imperative — no React hooks)
// ---------------------------------------------------------------------------

/** Generate presigned upload URLs for a batch of files. */
export function generateUploadUrls(
  files: UploadFileInput[],
): Promise<UploadSession> {
  return getConvexClient().action(api.uploads.generateUploadUrls, { files });
}
