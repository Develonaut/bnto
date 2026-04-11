# Optimize SVG Node

> Reduce SVG file size by removing unnecessary data and optimizing paths.

**Category:** vector | **Platforms:** browser | **Container:** no

## Accepts

- `image/svg+xml`

## Parameters

| Parameter      | Type    | Default | Range | Description                                                      |
| -------------- | ------- | ------- | ----- | ---------------------------------------------------------------- |
| precision      | number  | 3       | 1-10  | Decimal places for numeric values in paths and transforms (1-10) |
| removeComments | boolean | true    | -     | Strip XML comments                                               |
| removeMetadata | boolean | true    | -     | Strip <metadata> elements                                        |
| collapseGroups | boolean | true    | -     | Merge redundant nested <g> elements                              |
| minify         | boolean | true    | -     | Remove unnecessary whitespace and line breaks                    |

## Configuration Example

```json
{
  "type": "vector-optimize",
  "parameters": {
    "precision": 3,
    "removeComments": true,
    "removeMetadata": true,
    "collapseGroups": true,
    "minify": true
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
