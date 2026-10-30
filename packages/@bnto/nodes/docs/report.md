# Report Node

> Report a templated line in the run's output; files pass through unchanged.

**Category:** control | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| message | string | - | - | The line to print, usually a template. {{file}} (the file's name) or {{item.<key>}} prints it once per file (inside a loop, the loop's item wins a key both have), {{entry.<key>}} (or {{entry}}) once per entry of each, where {{item.<key>}} is the loop's item. |
| each | object | - | - | A list to print the message once per entry of, in order, usually a {{node.<id>.<key>}} template, or inside a loop the loop's {{item.<key>}}. |

## Configuration Example

```json
{
  "type": "report",
  "parameters": {
      "message": "<string>"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
