# Look Up Rows Node

> Attach the CSV row whose key matches each file's name to that file.

**Category:** spreadsheet | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| table | string | - | - | Path of the CSV (with a header row) to look each file up in |
| key_column | string | name | - | Table column matched against each file; a key may appear on one row only |
| match_on | enum | path | path, filename | Which part of the file's name must equal the key exactly |
| unmatched | enum | keep | keep, drop, fail | What happens to a file with no row: keep it (flagged unmatched: 1), drop it, or fail the run |

## Configuration Example

```json
{
  "type": "spreadsheet-lookup",
  "parameters": {
      "table": "<string>",
      "key_column": "name",
      "match_on": "path",
      "unmatched": "keep"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
