# Rename CSV Columns Node

> Rename column headers in a CSV file.

**Category:** spreadsheet | **Platforms:** browser | **Container:** no

## Accepts

- `text/csv`

## Parameters

| Parameter | Type   | Default | Range | Description                                                        |
| --------- | ------ | ------- | ----- | ------------------------------------------------------------------ |
| columns   | object | —       | —     | Map of old column names to new names (e.g., {"Name": "full_name"}) |

## Configuration Example

```json
{
  "type": "spreadsheet-rename",
  "parameters": {}
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
