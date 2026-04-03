/** Download Video recipe — download video from URLs using yt-dlp (CLI/desktop only). */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

export const downloadVideo: Recipe = {
  id: "d7e2c891-4f3a-4b1e-9c8d-2a5f6e7b3c01",
  slug: "download-video",
  name: "Download Video",
  description:
    "Download video or audio from URLs using yt-dlp. Supports YouTube, m3u8/HLS streams, and hundreds of sites. CLI/desktop only.",
  category: "video",
  accept: {
    mimeTypes: [],
    extensions: [],
    label: "No file input required",
  },
  features: ["YouTube", "m3u8/HLS", "MP4/WebM/MKV", "Audio extraction"],
  definition: {
    id: "download-video",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Download Video",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Downloads video or audio from a URL via yt-dlp.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode({
        accept: [],
        extensions: [],
        label: "No file input required",
      }),
      {
        id: "video-download",
        type: "video-download",
        version: CURRENT_FORMAT_VERSION,
        name: "Download Video",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          url: "",
          format: "mp4",
          quality: "best",
        },
        inputPorts: [],
        outputPorts: [],
      },
      defaultOutputNode({ label: "Downloaded Video" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "video-download" },
      { id: "e2", source: "video-download", target: "output" },
    ],
  },
};
