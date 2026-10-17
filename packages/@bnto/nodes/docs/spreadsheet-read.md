# Read Rows Node

> Explode a CSV or a JSON array into one item per row for loop iteration. Nested JSON fields read as dotted keys (meta.id).

**Category:** spreadsheet | **Platforms:** browser | **Container:** no

## Accepts

- `text/csv`
- `application/json`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| format | enum | auto | auto, csv, json | How the file is read: auto picks JSON for a .json file and CSV otherwise |
| hasHeaders | boolean | true | - | First row contains column headers (CSV only) |
| delimiter | enum | comma | comma, semicolon, tab, pipe | Column separator character (CSV only) |
| maxRows | number | 100000 | 1-10000000 | Maximum rows (CSV rows or JSON array elements) to process. Error if the file has more. |

## Configuration Example

```json
{
  "type": "spreadsheet-read",
  "parameters": {
      "format": "auto",
      "hasHeaders": true,
      "delimiter": "comma",
      "maxRows": 100000
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
