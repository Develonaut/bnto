# Join Files Node

> Join the files flowing in into one file, in arrival order (edge order when several steps feed it), a separator between them.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| filename | string | - | - | Name of the joined file, e.g. README.md; its MIME type follows the extension. Reads run templates such as {{vars.X}}. |
| separator | string |  | - | Put between files, as written (a newline, a blank line, ---). Not before the first or after the last. Empty joins them directly. |

## Configuration Example

```json
{
  "type": "file-join",
  "parameters": {
      "filename": "<string>",
      "separator": ""
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
