"use node";

import { v } from "convex/values";
import { internalAction } from "./_generated/server";
import {
  DeleteObjectsCommand,
  ListObjectsV2Command,
} from "@aws-sdk/client-s3";
import { createR2Client } from "./_helpers/r2_client";

/**
 * Delete all R2 objects under a given prefix.
 * Used for scheduled cleanup of transit files (inputs and outputs).
 *
 * Safe to call multiple times — deleting nonexistent objects is a no-op in S3.
 * Uses batch DeleteObjects (up to 1000 keys per request) to avoid rate limits.
 */
export const deleteByPrefix = internalAction({
  args: {
    prefix: v.string(),
  },
  handler: async (_ctx, args) => {
    const client = createR2Client();
    const bucket = process.env.R2_BUCKET_NAME ?? "bnto-transit";

    const listResponse = await client.send(
      new ListObjectsV2Command({
        Bucket: bucket,
        Prefix: args.prefix,
      }),
    );

    const objects = listResponse.Contents ?? [];
    if (objects.length === 0) return;

    await client.send(
      new DeleteObjectsCommand({
        Bucket: bucket,
        Delete: {
          Objects: objects.map((obj) => ({ Key: obj.Key })),
          Quiet: true,
        },
      }),
    );
  },
});
