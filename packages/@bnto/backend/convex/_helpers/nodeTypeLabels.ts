/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 *
 * Convex can't import from @bnto/nodes (bundling constraint), so this
 * file is generated alongside the main catalog. Run `task nodes:generate`
 * to regenerate after engine changes.
 *
 * Engine catalog v1.0.0
 */

/** Node type name → display label for processing nodes (excludes I/O and containers). */
export const NODE_TYPE_LABELS: Record<string, string> = {
  "assert": "Assert",
  "edit-fields": "Edit Fields",
  "file-collect": "Collect Files",
  "file-copy": "Copy Files",
  "file-decode": "Decode Files",
  "file-delete": "Delete Files",
  "file-filter": "Filter Files",
  "file-inspect": "Inspect Files",
  "file-join": "Join Files",
  "file-list": "List Files",
  "file-move": "Move Files",
  "file-rename": "Rename Files",
  "file-render": "Render Text File",
  "file-sort": "Sort Files",
  "http-request": "HTTP Request",
  "image-compress": "Compress Images",
  "image-convert": "Convert Image Format",
  "image-overlay": "Overlay Images",
  "image-pad": "Pad Images",
  "image-resize": "Resize Images",
  "image-strip-exif": "Strip EXIF",
  "report": "Report",
  "shell-run": "Run Shell Command",
  "spreadsheet-clean": "Clean CSV",
  "spreadsheet-convert": "Convert CSV to JSON",
  "spreadsheet-export": "Export Items as CSV",
  "spreadsheet-lookup": "Look Up Rows",
  "spreadsheet-merge": "Merge CSV",
  "spreadsheet-read": "Read Rows",
  "spreadsheet-rename": "Rename CSV Columns",
  "summarize": "Summarize",
  "transform": "Transform",
  "vector-optimize": "Optimize SVG",
  "vector-rasterize": "Rasterize SVG",
};
