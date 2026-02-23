// ---------------------------------------------------------------------------
// Upload types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** File metadata sent to the backend to request a presigned upload URL. */
export interface UploadFileInput {
  name: string;
  contentType: string;
  sizeBytes: number;
}

/** A single presigned URL returned by the backend. */
export interface PresignedUploadUrl {
  name: string;
  key: string;
  url: string;
}

/** Response from the backend after generating presigned URLs for a batch. */
export interface UploadSession {
  sessionId: string;
  urls: PresignedUploadUrl[];
  expiresAt: number;
}

/** Progress for a single file upload. */
export interface FileUploadProgress {
  fileName: string;
  loaded: number;
  total: number;
  status: "pending" | "uploading" | "completed" | "failed";
  error?: string;
}

/** Aggregate upload progress across all files in a session. */
export interface UploadProgress {
  sessionId: string | null;
  files: FileUploadProgress[];
  status: "idle" | "uploading" | "completed" | "failed";
}
