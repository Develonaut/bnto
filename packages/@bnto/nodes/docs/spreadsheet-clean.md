# Clean CSV Node

> Remove empty rows, trim whitespace, and deduplicate CSV data.

**Category:** spreadsheet | **Platforms:** browser | **Container:** no

## Accepts

- `text/csv`

## Parameters

| Parameter        | Type    | Default | Range | Description                                            |
| ---------------- | ------- | ------- | ----- | ------------------------------------------------------ |
| trimWhitespace   | boolean | true    | -     | Remove leading and trailing whitespace from every cell |
| removeEmptyRows  | boolean | true    | -     | Skip rows where every cell is blank                    |
| removeDuplicates | boolean | true    | -     | Remove duplicate rows, keeping the first occurrence    |

## Configuration Example

```json
{
  "type": "spreadsheet-clean",
  "parameters": {
    "trimWhitespace": true,
    "removeEmptyRows": true,
    "removeDuplicates": true
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
