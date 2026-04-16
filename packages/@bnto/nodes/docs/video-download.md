# Download Video Node

> Download video from URLs using yt-dlp (CLI/desktop only).

**Category:** video | **Platforms:** server | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| url | string | - | - | Video URL to download (YouTube, m3u8/HLS, or direct link). |
| format | enum | mp4 | [object Object], [object Object], [object Object], [object Object], [object Object], [object Object], [object Object] | Output format for the downloaded video or audio. |
| quality | enum | best | [object Object], [object Object], [object Object], [object Object], [object Object] | Maximum video quality. 'best' downloads the highest available. |
| args | string |  | - | Raw yt-dlp arguments, space-separated. Appended after built-in flags. |

## Configuration Example

```json
{
  "type": "video-download",
  "parameters": {
      "url": "<string>",
      "format": "mp4",
      "quality": "best",
      "args": ""
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
