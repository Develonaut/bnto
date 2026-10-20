# Report Node

> Print a templated line to the run's output; files pass through unchanged.

**Category:** control | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| message | string | - | - | The line to print, usually a template. {{item.<key>}} prints it once per file, {{entry.<key>}} (or {{entry}}) once per entry of each. |
| each | object | - | - | A list to print the message once per entry of, in order, usually a {{node.<id>.<key>}} template. |

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
