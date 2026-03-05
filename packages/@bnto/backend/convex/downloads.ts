"use node";

import { v } from "convex/values";
import { action } from "./_generated/server";
import { ConvexError } from "convex/values";
import { api } from "./_generated/api";
import { GetObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { createR2Client } from "./_helpers/r2_client";
import { sanitizeFileName } from "./_helpers/upload_validation";

/** Presigned download URLs are valid for 1 hour. */
const DOWNLOAD_URL_EXPIRY_SECONDS = 3600;

type OutputFile = {
  name: string;
  key: string;
  sizeBytes: number;
  contentType: string;
};

type OutputFileUrl = OutputFile & { url: string };

type DownloadUrlsResult = {
  urls: OutputFileUrl[];
  expiresAt: number;
};

/** Generate a presigned GET URL for a single output file. */
async function signOutputFile(
  client: ReturnType<typeof createR2Client>,
  bucket: string,
  file: OutputFile,
): Promise<OutputFileUrl> {
  const safeName = sanitizeFileName(file.name);
  const command = new GetObjectCommand({
    Bucket: bucket,
    Key: file.key,
    ResponseContentDisposition: `attachment; filename="${safeName}"`,
  });
  const url = await getSignedUrl(client, command, {
    expiresIn: DOWNLOAD_URL_EXPIRY_SECONDS,
  });
  return { ...file, url };
}

/**
 * Generate presigned GET URLs for downloading execution output files from R2.
 * Verifies the authenticated user owns the execution before generating URLs.
 */
export const generateDownloadUrls = action({
  args: {
    executionId: v.id("executions"),
  },
  handler: async (ctx, args): Promise<DownloadUrlsResult> => {
    const execution = await ctx.runQuery(api.executions.get, {
      id: args.executionId,
    });

    if (!execution) throw new ConvexError("Execution not found");
    if (execution.status !== "completed") {
      throw new ConvexError("Execution is not completed");
    }

    const outputFiles = execution.outputFiles;
    if (!outputFiles || outputFiles.length === 0) {
      return { urls: [] as OutputFileUrl[], expiresAt: 0 };
    }

    const client = createR2Client();
    const bucket = process.env.R2_BUCKET_NAME ?? "bnto-transit";
    const urls = await Promise.all(
      outputFiles.map((file) => signOutputFile(client, bucket, file)),
    );

    return {
      urls,
      expiresAt: Date.now() + DOWNLOAD_URL_EXPIRY_SECONDS * 1000,
    };
  },
});
