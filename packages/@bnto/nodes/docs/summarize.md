# Summarize Node

> Summarize items into one per group of fields, with its count and field totals.

**Category:** data | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| groupBy | string | - | - | Metadata fields to group items by, separated by commas; items whose values print the same share a group. Groups keep the order their first item arrived in. Items missing a field (or with it null or empty) group together, with that field null. Leave empty to summarize every item as one group. |
| sum | string | - | - | Numeric metadata fields to total per group, separated by commas; each total is written as sum_<field>. Numbers written as text count; blank values are skipped; any other value fails the run, naming the item. |

## Configuration Example

```json
{
  "type": "summarize",
  "parameters": {}
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
