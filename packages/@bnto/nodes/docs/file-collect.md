# Collect Files Node

> Traverse a directory and collect files matching a glob pattern into the pipeline.

**Category:** file | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| pattern | string | * | - | Glob pattern to match files (e.g., "*.jpg", "**/*.svg"). Default: "*" (all files). |
| recursive | boolean | true | - | Traverse subdirectories when collecting files. |
| flatten | boolean | true | - | Strip directory structure from output filenames. When true, all files appear as if in the same directory. |

## Configuration Example

```json
{
  "type": "file-collect",
  "parameters": {
      "pattern": "*",
      "recursive": true,
      "flatten": true
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
