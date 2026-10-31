# Render Text File Node

> Render one text file from a template: a header, one row per item, then a footer.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| header | string | - | - | Written once before the rows. Reads {{count}} (the number of items), run templates such as {{vars.X}} or {{node.<id>.<key>}}, and inside a loop the loop's {{item.KEY}}. |
| row | string | - | - | Written once per item. {{item.KEY}} is that item's metadata value KEY (inside a loop, over the loop's item, which {{loop.<key>}} reads as it was; a key neither has fails the run, naming the item); {{file}} is its file name. |
| footer | string | - | - | Written once after the rows. Reads what the header reads. |
| separator | string | \n | - | Put between rows. Header, rows and footer are each on their own line, and the file ends with a newline. |
| filename | string | - | - | Name of the file written, e.g. report.md (.md is text/markdown, anything else text/plain). Reads run templates, and inside a loop the loop's {{item.*}}. |
| escape | enum | none | none, markdown | How values put into a row are escaped; the template text never is. markdown puts a backslash before \ ` * _ [ ] < > \| and turns line breaks into <br>, so a value can't break a table cell. |

## Configuration Example

```json
{
  "type": "file-render",
  "parameters": {
      "row": "<string>",
      "separator": "\n",
      "filename": "<string>",
      "escape": "none"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
