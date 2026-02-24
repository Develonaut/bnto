"use node";

import { v } from "convex/values";
import { internalAction } from "./_generated/server";
import {
  DeleteObjectsCommand,
  ListObjectsV2Command,
} from "@aws-sdk/client-s3";
import { createR2Client } from "./_helpers/r2_client";

/**
 * Delete all R2 objects under a given prefix, handling pagination.
 * Loops with ContinuationToken until all objects are deleted.
 *
 * Returns the total number of objects deleted.
 */
async function deleteAllUnderPrefix(
  prefix: string,
): Promise<{ deleted: number; bucket: string }> {
  const client = createR2Client();
  const bucket = process.env.R2_BUCKET_NAME ?? "bnto-transit";

  let deleted = 0;
  let continuationToken: string | undefined;

  do {
    const listResponse = await client.send(
      new ListObjectsV2Command({
        Bucket: bucket,
        Prefix: prefix,
        ContinuationToken: continuationToken,
      }),
    );

    const objects = listResponse.Contents ?? [];
    if (objects.length > 0) {
      await client.send(
        new DeleteObjectsCommand({
          Bucket: bucket,
          Delete: {
            Objects: objects.map((obj) => ({ Key: obj.Key })),
            Quiet: true,
          },
        }),
      );
      deleted += objects.length;
    }

    continuationToken = listResponse.IsTruncated
      ? listResponse.NextContinuationToken
      : undefined;
  } while (continuationToken);

  return { deleted, bucket };
}

/**
 * Delete all R2 objects under a given prefix.
 * Used for scheduled cleanup of transit files (inputs and outputs).
 *
 * Safe to call multiple times — deleting nonexistent objects is a no-op in S3.
 * Handles pagination via ContinuationToken for prefixes with >1000 objects.
 */
export const deleteByPrefix = internalAction({
  args: {
    prefix: v.string(),
  },
  handler: async (_ctx, args) => {
    await deleteAllUnderPrefix(args.prefix);
  },
});

/**
 * Purge all objects from the R2 transit bucket.
 * Intended for dev/test cleanup (e.g., after integration test runs).
 *
 * Calls deleteAllUnderPrefix with an empty prefix to match everything.
 */
export const purgeAll = internalAction({
  args: {},
  handler: async (_ctx) => {
    const { deleted, bucket } = await deleteAllUnderPrefix("");
    console.log(`purgeAll: deleted ${deleted} objects from ${bucket}`);
    return { deleted, bucket };
  },
});
