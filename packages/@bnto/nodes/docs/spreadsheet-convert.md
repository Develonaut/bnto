# CSV to JSON Node

> Convert CSV data to JSON format with configurable delimiters.

**Category:** spreadsheet | **Platforms:** browser | **Container:** no

## Accepts

- `text/csv`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| delimiter | enum | comma | [object Object], [object Object], [object Object], [object Object] | Column separator character |
| pretty | boolean | false | - | Format output JSON with indentation |

## Configuration Example

```json
{
  "type": "spreadsheet-convert",
  "parameters": {
      "delimiter": "comma",
      "pretty": false
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
