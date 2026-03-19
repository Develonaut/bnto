# File System Node

> File operations: rename files with find/replace, case transforms, and patterns.

**Category:** file | **Platforms:** browser | **Container:** no

## Operations

### rename

Transform filenames using patterns, find/replace, and case rules

## Parameters

| Parameter | Type   | Default | Range               | Visible When | Description                                                                        |
| --------- | ------ | ------- | ------------------- | ------------ | ---------------------------------------------------------------------------------- |
| operation | enum   | —       | rename              | always       | Processing operation                                                               |
| find      | string | —       | —                   | rename       | Text or regex pattern to search for in the filename                                |
| replace   | string | —       | —                   | rename       | Replacement text (used with Find)                                                  |
| case      | enum   | —       | lower, upper, title | rename       | Transform the filename to a specific case                                          |
| prefix    | string | —       | —                   | rename       | Text to prepend to the filename                                                    |
| suffix    | string | —       | —                   | rename       | Text to append before the file extension                                           |
| pattern   | string | —       | —                   | rename       | Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}}) |

## Configuration Example

```json
{
  "type": "file-system",
  "parameters": {
    "operation": "rename"
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
