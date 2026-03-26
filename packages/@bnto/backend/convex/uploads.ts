"use node";

import { v } from "convex/values";
import { action } from "./_generated/server";
import { ConvexError } from "convex/values";
import { api } from "./_generated/api";
import { PutObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { randomUUID } from "crypto";
import {
  validateUploadBatch,
  sanitizeFileName,
  PRESIGNED_URL_EXPIRY_SECONDS,
} from "./_helpers/upload_validation";
import { createR2Client } from "./_helpers/r2_client";

/** Generate a presigned PUT URL for a single file in R2. */
async function generateFileUrl(
  client: ReturnType<typeof createR2Client>,
  bucket: string,
  sessionId: string,
  file: { name: string; contentType: string },
) {
  const safeName = sanitizeFileName(file.name);
  const key = `uploads/${sessionId}/${safeName}`;
  const command = new PutObjectCommand({ Bucket: bucket, Key: key, ContentType: file.contentType });
  const url = await getSignedUrl(client, command, { expiresIn: PRESIGNED_URL_EXPIRY_SECONDS });
  return { name: file.name, key, url };
}

/**
 * Generate presigned PUT URLs for uploading files to R2.
 * Validates file types and enforces per-plan size limits.
 */
export const generateUploadUrls = action({
  args: {
    files: v.array(v.object({ name: v.string(), contentType: v.string(), sizeBytes: v.number() })),
  },
  handler: async (ctx, args) => {
    const user = await ctx.runQuery(api.users.getMe);
    if (!user) throw new ConvexError("Authentication required to upload files");

    const error = validateUploadBatch(args.files, user.plan);
    if (error) throw new ConvexError(error);

    const sessionId = randomUUID();
    const client = createR2Client();
    const bucket = process.env.R2_BUCKET_NAME ?? "bnto-transit";
    const urls = await Promise.all(
      args.files.map((f) => generateFileUrl(client, bucket, sessionId, f)),
    );
    return { sessionId, urls, expiresAt: Date.now() + PRESIGNED_URL_EXPIRY_SECONDS * 1000 };
  },
});
