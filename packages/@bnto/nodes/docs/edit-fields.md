# Edit Fields Node

> Edit fields on each item, setting them from templates over its other fields; files pass through.

**Category:** data | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| values | object | - | - | Field name to value. A value can read the item's other fields, e.g. {{item.first}} {{item.last}}, and {{file}} (the file's name); one that is exactly {{item.<key>}} keeps that field's type. Templates read the fields the item arrived with, and one it lacks fails the run. |
| keepOnlySet | boolean | false | - | When on, each item keeps only the fields set in values; every other field it carried is dropped. |

## Configuration Example

```json
{
  "type": "edit-fields",
  "parameters": {
      "values": "<object>",
      "keepOnlySet": false
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
