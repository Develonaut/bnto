# Optimize SVG Node

> Optimize SVG files by removing editor metadata, comments, and unnecessary elements.

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

## Configuration Example

```json
{
  "type": "vector-optimize",
  "parameters": {
      "removeComments": true,
      "removeMetadata": true,
      "collapseGroups": true,
      "minify": true
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
