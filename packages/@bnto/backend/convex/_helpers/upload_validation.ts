/**
 * Pure validation functions for file uploads to R2.
 * Enforces file type allowlists and per-plan size limits.
 */

/** MIME types allowed for upload. */
export const ALLOWED_MIME_TYPES = new Set([
  // Images
  "image/png",
  "image/jpeg",
  "image/webp",
  "image/gif",
  "image/svg+xml",
  "image/tiff",
  "image/bmp",
  // CSV / Spreadsheet
  "text/csv",
  "text/tab-separated-values",
  "application/vnd.ms-excel",
  "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  // Text / JSON
  "text/plain",
  "application/json",
]);

/** Maximum file size per plan tier, in bytes. */
const MAX_FILE_SIZE: Record<string, number> = {
  free: 25 * 1024 * 1024, // 25 MB
  starter: 100 * 1024 * 1024, // 100 MB
  pro: 500 * 1024 * 1024, // 500 MB
};

/** Maximum files per upload batch. */
export const MAX_UPLOAD_BATCH_SIZE = 50;

/** Maximum presigned URL validity period, in seconds. */
export const PRESIGNED_URL_EXPIRY_SECONDS = 900; // 15 minutes

export function isAllowedMimeType(contentType: string): boolean {
  return ALLOWED_MIME_TYPES.has(contentType);
}

export function getMaxFileSize(plan: string): number {
  return MAX_FILE_SIZE[plan] ?? MAX_FILE_SIZE.free;
}

export function isFileSizeAllowed(sizeBytes: number, plan: string): boolean {
  return sizeBytes > 0 && sizeBytes <= getMaxFileSize(plan);
}

/** Input shape for a file in an upload batch. */
export type UploadFileInput = {
  name: string;
  contentType: string;
  sizeBytes: number;
};

/** Structured validation error for upload batches. */
export type UploadValidationError = {
  code:
    | "EMPTY_BATCH"
    | "BATCH_TOO_LARGE"
    | "INVALID_FILE_NAME"
    | "INVALID_FILE_TYPE"
    | "FILE_TOO_LARGE";
  message: string;
  fileName?: string;
  maxSizeBytes?: number;
};

/**
 * Validate an entire upload batch against plan limits.
 * Returns the first validation error found, or null if all files pass.
 */
export function validateUploadBatch(
  files: UploadFileInput[],
  plan: string,
): UploadValidationError | null {
  if (files.length === 0) {
    return { code: "EMPTY_BATCH", message: "At least one file is required" };
  }
  if (files.length > MAX_UPLOAD_BATCH_SIZE) {
    return {
      code: "BATCH_TOO_LARGE",
      message: `Maximum ${MAX_UPLOAD_BATCH_SIZE} files per upload batch`,
    };
  }

  const maxSizeBytes = getMaxFileSize(plan);
  const maxSizeMB = Math.round(maxSizeBytes / (1024 * 1024));

  for (const file of files) {
    if (!file.name.trim()) {
      return {
        code: "INVALID_FILE_NAME",
        message: "File name cannot be empty",
      };
    }
    if (!isAllowedMimeType(file.contentType)) {
      return {
        code: "INVALID_FILE_TYPE",
        message: `File type "${file.contentType}" is not supported`,
        fileName: file.name,
      };
    }
    if (!isFileSizeAllowed(file.sizeBytes, plan)) {
      return {
        code: "FILE_TOO_LARGE",
        message: `File "${file.name}" exceeds the ${maxSizeMB}MB limit for the ${plan} plan`,
        fileName: file.name,
        maxSizeBytes,
      };
    }
  }

  return null;
}

/**
 * Sanitize a filename for use as an R2 object key segment.
 * Removes path traversal, special characters, and ensures a valid key.
 */
export function sanitizeFileName(name: string): string {
  // Strip path components (no traversal)
  const base = name.split(/[/\\]/).pop() ?? "";
  // Replace non-safe characters with underscores
  const sanitized = base.replace(/[^a-zA-Z0-9._-]/g, "_");
  // Collapse consecutive underscores
  const collapsed = sanitized.replace(/_{2,}/g, "_");
  // Remove underscores adjacent to dots (clean up "name_.ext" → "name.ext")
  const cleanDots = collapsed.replace(/_\./g, ".").replace(/\._/g, ".");
  // Remove leading/trailing underscores and dots (no hidden files)
  const trimmed = cleanDots.replace(/^[_.]+|[_.]+$/g, "");
  return trimmed || "file";
}
