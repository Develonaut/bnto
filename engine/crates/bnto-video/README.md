# bnto-video

Video operation nodes. Download videos via yt-dlp.

## Overview

`bnto-video` provides a `NodeProcessor` for downloading videos from URLs (YouTube, HLS/m3u8, direct links). Wraps yt-dlp via an adapter layer. CLI/desktop only — browser (WASM) has no shell access. Gated behind the `native` feature in `bnto-engine`.

## Processors

| Processor       | Node Type        | What It Does                                                    |
| --------------- | ---------------- | --------------------------------------------------------------- |
| `VideoDownload` | `video-download` | Download video from URL via yt-dlp with format/quality controls |

## Directory Structure

```
src/
├── lib.rs            # Public exports
├── download.rs       # VideoDownload processor + metadata
└── ytdlp.rs          # yt-dlp adapter (command building, output parsing)
```

## Development

```bash
cargo test -p bnto-video           # Native unit tests
```
