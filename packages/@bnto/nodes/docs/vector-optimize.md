# Optimize SVG Node

> Remove editor metadata, comments, and unnecessary elements from SVG files.

**Category:** vector | **Platforms:** browser | **Container:** no

## Accepts

- `image/svg+xml`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| removeComments | boolean | true | - | Strip XML comments from the SVG |
| removeMetadata | boolean | true | - | Strip <metadata> elements (RDF, Dublin Core, etc.) |
| collapseGroups | boolean | true | - | Remove empty groups and collapse single-child wrapper groups |
| minify | boolean | true | - | Remove unnecessary whitespace and newlines |
| precision | number | 3 | 0-8 | Decimal places for numeric values (reserved for Tier 2) |

## Configuration Example

```json
{
  "type": "vector-optimize",
  "parameters": {
      "removeComments": true,
      "removeMetadata": true,
      "collapseGroups": true,
      "minify": true,
      "precision": 3
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
