# Shell Command Node

> Execute shell commands with stall detection, retry, and streaming output.

**Category:** system | **Platforms:** server | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| command | string | - | - | Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH. |
| args | string | - | - | Command arguments as an array of strings. |
| outputMode | string | stdout | - | How to collect output. 'stdout' captures command output. 'file' reads files written by the command to a temp directory (use {output_dir} in args to inject the path). |
| timeout | number | 300 | - | Maximum execution time in seconds. Default: 300. |
| env | object | - | - | Additional environment variables for the command. |

## Configuration Example

```json
{
  "type": "shell-command",
  "parameters": {
      "outputMode": "stdout",
      "timeout": 300
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
