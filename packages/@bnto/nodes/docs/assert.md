# Assert Node

> Fail the run when a comparison doesn't hold; otherwise pass files through.

**Category:** control | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| value | string | - | - | What to check, usually a template such as {{node.<id>.<key>}} or {{item.<key>}}. |
| op | enum | equals | equals, not_equals, after, before | How value is compared to expected. after and before compare RFC 3339 times. |
| expected | string | - | - | What the value is compared to. |
| message | string | - | - | Put before the reasons when the check fails, so the failure says what was checked. |

## Configuration Example

```json
{
  "type": "assert",
  "parameters": {
      "op": "equals"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
