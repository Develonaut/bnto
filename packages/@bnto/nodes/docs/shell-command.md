# Shell Command Node

> Execute shell commands with stall detection, retry, and streaming output. Still runs under bnto run --dry-run, so a recipe run dry uses it only to read.

**Category:** system | **Platforms:** server | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| command | string | - | - | Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH. |
| args | string | - | - | Command arguments: text, conditional groups (nested lists, dropped when any part is empty) and {"each": [...]} groups, repeated once per incoming file with {{path}}, {{file}} and {{item.KEY}} filled. |
| outputMode | string | stdout | - | How to collect output. 'stdout' captures command output. 'file' reads files written by the command to a temp directory (use {{output_dir}} in args to inject the path). |
| timeout | number | 300 | - | Maximum execution time in seconds. Default: 300. |
| maxOutputSize | number | 500 | - | Maximum size per output file in megabytes. Default: 100 MB. |
| env | object | - | - | Additional environment variables for the command. |
| search_path | object | - | - | Folders to find the command in, in order, instead of PATH. The command is still a bare name and runs by its full path; relative folders are under the working directory. |
| cwd | string | - | - | Folder to run the command in, under the working directory (e.g. 'Tools'). Relative args are relative to it; search_path and {{output_dir}} are not. Absolute paths and ".." are refused. |

## Configuration Example

```json
{
  "type": "shell-command",
  "parameters": {
      "command": "<string>",
      "outputMode": "stdout",
      "timeout": 300,
      "maxOutputSize": 500
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
