# Rename Files Node

> Transform filenames using patterns, find/replace, case rules, and counters.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter     | Type   | Default | Range               | Description                                                                                     |
| ------------- | ------ | ------- | ------------------- | ----------------------------------------------------------------------------------------------- |
| find          | string | -       | -                   | Text or regex pattern to search for in the filename                                             |
| replace       | string | -       | -                   | Replacement text (used with Find)                                                               |
| case          | enum   | -       | lower, upper, title | Transform the filename to a specific case                                                       |
| prefix        | string | -       | -                   | Text to prepend to the filename                                                                 |
| suffix        | string | -       | -                   | Text to append before the file extension                                                        |
| pattern       | string | -       | -                   | Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}}, {{counter}}) |
| counter_start | number | 1       | ≥ 0                 | Starting number for the {{counter}} variable                                                    |
| counter_pad   | number | 0       | 0-10                | Zero-pad width for the counter (e.g., 3 → 001, 002)                                             |
| extension     | string | -       | -                   | Replace the file extension (without dot)                                                        |

## Configuration Example

```json
{
  "type": "file-rename",
  "parameters": {
    "counter_start": 1,
    "counter_pad": 0
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
