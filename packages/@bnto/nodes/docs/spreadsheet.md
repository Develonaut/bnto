# Spreadsheet Node

> Spreadsheet operations: clean data and rename columns.

**Category:** spreadsheet | **Platforms:** browser | **Container:** no

## Accepts

- `text/csv`

## Operations

### clean

Remove empty rows, trim whitespace, and deduplicate CSV data

Accepts: `text/csv`

### rename

Rename column headers in a CSV file

Accepts: `text/csv`

## Parameters

| Parameter        | Type    | Default | Range         | Visible When | Description                                                        |
| ---------------- | ------- | ------- | ------------- | ------------ | ------------------------------------------------------------------ |
| operation        | enum    | —       | clean, rename | always       | Processing operation                                               |
| trimWhitespace   | boolean | true    | —             | clean        | Remove leading and trailing whitespace from every cell             |
| removeEmptyRows  | boolean | true    | —             | clean        | Skip rows where every cell is blank                                |
| removeDuplicates | boolean | true    | —             | clean        | Remove duplicate rows, keeping the first occurrence                |
| columns          | object  | —       | —             | rename       | Map of old column names to new names (e.g., {"Name": "full_name"}) |

## Configuration Example

```json
{
  "type": "spreadsheet",
  "parameters": {
    "operation": "clean",
    "trimWhitespace": true,
    "removeEmptyRows": true,
    "removeDuplicates": true
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
