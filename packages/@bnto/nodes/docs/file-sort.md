# Sort Files Node

> Reorder items by one or more metadata fields, each ascending or descending.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| by | string | - | - | Metadata fields to sort by, separated by commas; later fields break ties. A leading '-' sorts that field descending. Numbers sort numerically; items missing a field (or with it null or empty) go last. |
| case | enum | sensitive | sensitive, insensitive | How text compares: as written, by Unicode code point (upper case before lower), or ignoring case. |

## Configuration Example

```json
{
  "type": "file-sort",
  "parameters": {
      "by": "<string>",
      "case": "sensitive"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
