/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for video-download. */
export const videoDownloadProcessor: ProcessorDef = {
  nodeType: "video-download",
  name: "Download Video",
  description: "Download video from URLs using yt-dlp.",
  category: "video",
  accepts: [] as const,
  platforms: ["cli","server","desktop"] as const,
  parameters: [
  {
    name: "url",
    label: "URL",
    description: "Video URL to download (YouTube, m3u8/HLS, or direct link).",
    type: "string" as const,
    constraints: { required: true },
    placeholder: "https://youtube.com/watch?v=...",
  },
  {
    name: "format",
    label: "Format",
    description: "Output format for the downloaded video or audio.",
    type: "enum" as const,
    options: [{"value":"mp4","label":"MP4"},{"value":"webm","label":"WebM"},{"value":"mkv","label":"MKV"},{"value":"mp3","label":"MP3"},{"value":"m4a","label":"M4A"},{"value":"wav","label":"WAV"},{"value":"flac","label":"FLAC"}] as const,
    default: "mp4",
  },
  {
    name: "quality",
    label: "Quality",
    description: "Maximum video quality. 'best' downloads the highest available.",
    type: "enum" as const,
    options: [{"value":"best","label":"Best Available"},{"value":"1080","label":"1080p"},{"value":"720","label":"720p"},{"value":"480","label":"480p"},{"value":"360","label":"360p"}] as const,
    default: "best",
  },
  {
    name: "args",
    label: "Extra Arguments",
    description: "Raw yt-dlp arguments, space-separated. Appended after built-in flags.",
    type: "string" as const,
    default: "",
    placeholder: "--cookies-from-browser chrome",
    surfaceable: false,
  },
  ],
  inputCardinality: "source" as const,
};
