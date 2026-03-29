/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 *
 * Convex can't import from @bnto/nodes (bundling constraint), so this
 * file is generated alongside the main catalog. Run `task nodes:generate`
 * to regenerate after engine changes.
 *
 * Engine catalog v1.0.0
 */

/** Node type name → display label for processing nodes (excludes I/O and containers). */
export const NODE_TYPE_LABELS: Record<string, string> = {
  "edit-fields": "Edit Fields",
  "file-rename": "Rename Files",
  "http-request": "HTTP Request",
  "image-compress": "Compress Images",
  "image-convert": "Convert Image Format",
  "image-resize": "Resize Images",
  "image-strip-exif": "Strip EXIF",
  "image-watermark": "Watermark",
  "shell-command": "Shell Command",
  "spreadsheet-clean": "Clean CSV",
  "spreadsheet-convert": "CSV to JSON",
  "spreadsheet-merge": "Merge CSV",
  "spreadsheet-rename": "Rename CSV Columns",
  transform: "Transform",
};
