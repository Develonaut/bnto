# Merge CSV Node

> Combine multiple CSV files into one with header reconciliation and deduplication.

**Category:** spreadsheet | **Platforms:** browser | **Container:** no

## Accepts

- `text/csv`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| headerHandling | enum | first-file | [object Object], [object Object] | How to reconcile headers across files |
| deduplicate | boolean | false | - | Remove duplicate rows across all files |

## Configuration Example

```json
{
  "type": "spreadsheet-merge",
  "parameters": {
      "headerHandling": "first-file",
      "deduplicate": false
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
