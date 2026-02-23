// ---------------------------------------------------------------------------
// Download types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** A single output file with its presigned download URL. */
export interface OutputFileUrl {
  name: string;
  key: string;
  sizeBytes: number;
  contentType: string;
  url: string;
}

/** Response from the backend with presigned download URLs. */
export interface DownloadUrlsResult {
  urls: OutputFileUrl[];
  expiresAt: number;
}
