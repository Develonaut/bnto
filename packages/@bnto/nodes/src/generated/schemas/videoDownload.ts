/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for video-download node parameters. */
export const videoDownloadParamsSchema = z.object({
    url: z.string(),
    format: z.enum([{"value":"mp4","label":"MP4"},{"value":"webm","label":"WebM"},{"value":"mkv","label":"MKV"},{"value":"mp3","label":"MP3"},{"value":"m4a","label":"M4A"},{"value":"wav","label":"WAV"},{"value":"flac","label":"FLAC"}] as const).optional().default("mp4"),
    quality: z.enum([{"value":"best","label":"Best Available"},{"value":"1080","label":"1080p"},{"value":"720","label":"720p"},{"value":"480","label":"480p"},{"value":"360","label":"360p"}] as const).optional().default("best"),
    args: z.string().optional().default(""),
});

/** Inferred TypeScript type for video-download node parameters. */
export type VideoDownloadParams = z.infer<typeof videoDownloadParamsSchema>;

/** Full schema definition for the video-download node type. */
export const videoDownloadNodeSchema: NodeSchema = {
  nodeType: "video-download",
  schemaVersion: 1,
  schema: videoDownloadParamsSchema,
  params: {
    url: {
      label: "URL",
      description: "Video URL to download (YouTube, m3u8/HLS, or direct link).",
      placeholder: "https://youtube.com/watch?v=...",
    },
    format: {
      label: "Format",
      description: "Output format for the downloaded video or audio.",
    },
    quality: {
      label: "Quality",
      description: "Maximum video quality. 'best' downloads the highest available.",
    },
    args: {
      label: "Extra Arguments",
      description: "Raw yt-dlp arguments, space-separated. Appended after built-in flags.",
      placeholder: "--cookies-from-browser chrome",
      surfaceable: false,
    },
  },
};
