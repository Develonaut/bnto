# Read CSV Rows Node

> Explode a CSV into one item per row for loop iteration.

**Category:** spreadsheet | **Platforms:** browser | **Container:** no

## Accepts

- `text/csv`

## Parameters

| Parameter  | Type    | Default | Range                       | Description                                               |
| ---------- | ------- | ------- | --------------------------- | --------------------------------------------------------- |
| hasHeaders | boolean | true    | -                           | First row contains column headers                         |
| delimiter  | enum    | comma   | comma, semicolon, tab, pipe | Column separator character                                |
| maxRows    | number  | 100000  | 1-10000000                  | Maximum rows to process. Error if CSV exceeds this limit. |

## Configuration Example

```json
{
  "type": "spreadsheet-read",
  "parameters": {
    "hasHeaders": true,
    "delimiter": "comma",
    "maxRows": 100000
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
