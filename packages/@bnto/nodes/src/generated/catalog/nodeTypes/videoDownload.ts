/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for video-download. */
export const videoDownloadNodeType: NodeTypeInfo = {
  name: "video-download",
  label: "Download Video",
  description: "Download video from URLs using yt-dlp (CLI/desktop only).",
  category: "video",
  isContainer: false,
  platforms: ["server"] as const,
  icon: "video",
};
