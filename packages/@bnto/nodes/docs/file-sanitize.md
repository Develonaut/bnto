# Sanitize Filenames Node

> Clean filenames for web-safe or cross-platform use.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter  | Type   | Default | Range                     | Description                                        |
| ---------- | ------ | ------- | ------------------------- | -------------------------------------------------- |
| mode       | enum   | slugify | slugify, strip, normalize | Sanitization strategy                              |
| separator  | string | -       | -                         | Character to replace spaces and special characters |
| max_length | number | 0       | ≥ 0                       | Maximum filename length (0 = no limit)             |

## Configuration Example

```json
{
  "type": "file-sanitize",
  "parameters": {
    "mode": "slugify",
    "separator": "-",
    "max_length": 0
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
