/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { Definition } from "../definition";

/** A generated recipe from the engine's built-in catalog. */
export interface GeneratedRecipe {
  readonly slug: string;
  readonly name: string;
  readonly description: string;
  readonly category: string;
  readonly tags: readonly string[];
  readonly definition: Definition;
}

/** All engine-owned built-in recipes. */
export const GENERATED_RECIPES: readonly GeneratedRecipe[] = [
  {
    slug: "compress-images",
    name: "Compress Images",
    description:
      "Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.",
    category: "image",
    tags: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "compress-image",
        },
        {
          id: "e2",
          source: "compress-image",
          target: "output",
        },
      ],
      id: "compress-images",
      inputPorts: [],
      metadata: {
        category: "image",
        description:
          "Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.",
        tags: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
      },
      name: "Compress Images",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp"],
            extensions: [".jpg", ".jpeg", ".png", ".webp"],
            label: "JPEG, PNG, or WebP images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "compress-image",
          inputPorts: [],
          metadata: {},
          name: "Compress",
          outputPorts: [],
          parameters: {
            quality: 80,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-compress",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-compress-images",
            label: "Compressed Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "resize-images",
    name: "Resize Images",
    description: "Resize images to exact dimensions or percentages. Free, no signup required.",
    category: "image",
    tags: ["PNG", "JPEG", "WebP", "Custom dimensions", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "resize-image",
        },
        {
          id: "e2",
          source: "resize-image",
          target: "output",
        },
      ],
      id: "resize-images",
      inputPorts: [],
      metadata: {
        category: "image",
        description: "Resize images to exact dimensions or percentages. Free, no signup required.",
        tags: ["PNG", "JPEG", "WebP", "Custom dimensions", "Browser-based"],
      },
      name: "Resize Images",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp"],
            extensions: [".jpg", ".jpeg", ".png", ".webp"],
            label: "JPEG, PNG, or WebP images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "resize-image",
          inputPorts: [],
          metadata: {},
          name: "Resize",
          outputPorts: [],
          parameters: {
            maintainAspect: true,
            quality: 80,
            width: 200,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-resize",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-resize-images",
            label: "Resized Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "convert-image-format",
    name: "Convert Image Format",
    description: "Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.",
    category: "image",
    tags: ["PNG", "JPEG", "WebP", "GIF", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "convert-image",
        },
        {
          id: "e2",
          source: "convert-image",
          target: "output",
        },
      ],
      id: "convert-image-format",
      inputPorts: [],
      metadata: {
        category: "image",
        description: "Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.",
        tags: ["PNG", "JPEG", "WebP", "GIF", "Browser-based"],
      },
      name: "Convert Image Format",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp", "image/gif"],
            extensions: [".jpg", ".jpeg", ".png", ".webp", ".gif"],
            label: "JPEG, PNG, WebP, or GIF images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "convert-image",
          inputPorts: [],
          metadata: {},
          name: "Convert",
          outputPorts: [],
          parameters: {
            format: "webp",
            quality: 80,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-convert",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-convert-image-format",
            label: "Converted Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "rename-files",
    name: "Rename Files",
    description: "Batch rename files with patterns. Free, no signup required.",
    category: "file",
    tags: ["Batch rename", "Pattern matching", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "rename-file",
        },
        {
          id: "e2",
          source: "rename-file",
          target: "output",
        },
      ],
      id: "rename-files",
      inputPorts: [],
      metadata: {
        category: "file",
        description: "Batch rename files with patterns. Free, no signup required.",
        tags: ["Batch rename", "Pattern matching", "Browser-based"],
      },
      name: "Rename Files",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["*/*"],
            extensions: [],
            label: "any files",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "rename-file",
          inputPorts: [],
          metadata: {},
          name: "Rename",
          outputPorts: [],
          parameters: {
            prefix: "renamed-",
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "file-rename",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-rename-files",
            label: "Renamed Files",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "clean-csv",
    name: "Clean CSV",
    description: "Remove empty rows, trim whitespace, deduplicate CSV data. Free, no signup.",
    category: "spreadsheet",
    tags: ["CSV", "Remove duplicates", "Trim whitespace", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "clean",
        },
        {
          id: "e2",
          source: "clean",
          target: "output",
        },
      ],
      id: "clean-csv",
      inputPorts: [],
      metadata: {
        category: "spreadsheet",
        description: "Remove empty rows, trim whitespace, deduplicate CSV data. Free, no signup.",
        tags: ["CSV", "Remove duplicates", "Trim whitespace", "Browser-based"],
      },
      name: "Clean CSV",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["text/csv"],
            extensions: [".csv"],
            label: "CSV files",
            mode: "file-upload",
            multiple: false,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "clean",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Clean",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            removeDuplicates: true,
            removeEmptyRows: true,
            trimWhitespace: true,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "spreadsheet-clean",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-clean-csv",
            label: "Cleaned CSV",
            mode: "download",
            zip: false,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "rename-csv-columns",
    name: "Rename CSV Columns",
    description: "Rename CSV column headers in bulk. Free, no signup required.",
    category: "spreadsheet",
    tags: ["CSV", "Column rename", "Bulk edit", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "rename-columns",
        },
        {
          id: "e2",
          source: "rename-columns",
          target: "output",
        },
      ],
      id: "rename-csv-columns",
      inputPorts: [],
      metadata: {
        category: "spreadsheet",
        description: "Rename CSV column headers in bulk. Free, no signup required.",
        tags: ["CSV", "Column rename", "Bulk edit", "Browser-based"],
      },
      name: "Rename CSV Columns",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["text/csv"],
            extensions: [".csv"],
            label: "CSV files",
            mode: "file-upload",
            multiple: false,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "rename-columns",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Rename",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            columns: {},
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "spreadsheet-rename",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-rename-csv-columns",
            label: "Renamed CSV",
            mode: "download",
            zip: false,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "csv-to-json",
    name: "CSV to JSON",
    description: "Convert CSV files to JSON format with configurable delimiters. Free, no signup.",
    category: "spreadsheet",
    tags: ["CSV", "JSON", "Configurable delimiter", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "convert",
        },
        {
          id: "e2",
          source: "convert",
          target: "output",
        },
      ],
      id: "csv-to-json",
      inputPorts: [],
      metadata: {
        category: "spreadsheet",
        description:
          "Convert CSV files to JSON format with configurable delimiters. Free, no signup.",
        tags: ["CSV", "JSON", "Configurable delimiter", "Browser-based"],
      },
      name: "CSV to JSON",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["text/csv"],
            extensions: [".csv"],
            label: "CSV files",
            mode: "file-upload",
            multiple: false,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "convert",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Convert to JSON",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            delimiter: "comma",
            pretty: false,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "spreadsheet-convert",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-csv-to-json",
            label: "JSON file",
            mode: "download",
            zip: false,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "merge-csv",
    name: "Merge CSV",
    description: "Combine multiple CSV files into one with header reconciliation. Free, no signup.",
    category: "spreadsheet",
    tags: ["CSV", "Merge", "Header reconciliation", "Deduplication", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "merge",
        },
        {
          id: "e2",
          source: "merge",
          target: "output",
        },
      ],
      id: "merge-csv",
      inputPorts: [],
      metadata: {
        category: "spreadsheet",
        description:
          "Combine multiple CSV files into one with header reconciliation. Free, no signup.",
        tags: ["CSV", "Merge", "Header reconciliation", "Deduplication", "Browser-based"],
      },
      name: "Merge CSV",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["text/csv"],
            extensions: [".csv"],
            label: "CSV files",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "merge",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Merge",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            deduplicate: false,
            headerHandling: "first-file",
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "spreadsheet-merge",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-merge-csv",
            label: "Merged CSV",
            mode: "download",
            zip: false,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "optimize-images-for-web",
    name: "Optimize Images for Web",
    description:
      "Resize, convert to WebP, and compress images for fast web loading. Free, no signup.",
    category: "image",
    tags: ["Resize", "WebP", "Compress", "Multi-step", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "resize",
        },
        {
          id: "e2",
          source: "resize",
          target: "convert",
        },
        {
          id: "e3",
          source: "convert",
          target: "compress",
        },
        {
          id: "e4",
          source: "compress",
          target: "output",
        },
      ],
      id: "optimize-images-for-web",
      inputPorts: [],
      metadata: {
        category: "image",
        description:
          "Resize, convert to WebP, and compress images for fast web loading. Free, no signup.",
        tags: ["Resize", "WebP", "Compress", "Multi-step", "Browser-based"],
      },
      name: "Optimize Images for Web",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp"],
            extensions: [".jpg", ".jpeg", ".png", ".webp"],
            label: "JPEG, PNG, or WebP images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "resize",
          inputPorts: [],
          metadata: {},
          name: "Resize",
          outputPorts: [],
          parameters: {
            maintainAspect: true,
            quality: 80,
            width: 800,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-resize",
          version: "1.0.0",
        },
        {
          id: "convert",
          inputPorts: [],
          metadata: {},
          name: "Convert",
          outputPorts: [],
          parameters: {
            format: "webp",
            quality: 80,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "image-convert",
          version: "1.0.0",
        },
        {
          id: "compress",
          inputPorts: [],
          metadata: {},
          name: "Compress",
          outputPorts: [],
          parameters: {
            quality: 80,
          },
          position: {
            x: 750,
            y: 100,
          },
          type: "image-compress",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-optimize-images-for-web",
            label: "Optimized Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 1000,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "generate-thumbnails",
    name: "Generate Thumbnails",
    description:
      "Resize images to thumbnail size, convert to WebP, and add a prefix. Free, no signup.",
    category: "image",
    tags: ["Thumbnails", "Resize", "WebP", "Multi-step", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "resize",
        },
        {
          id: "e2",
          source: "resize",
          target: "convert",
        },
        {
          id: "e3",
          source: "convert",
          target: "rename",
        },
        {
          id: "e4",
          source: "rename",
          target: "output",
        },
      ],
      id: "generate-thumbnails",
      inputPorts: [],
      metadata: {
        category: "image",
        description:
          "Resize images to thumbnail size, convert to WebP, and add a prefix. Free, no signup.",
        tags: ["Thumbnails", "Resize", "WebP", "Multi-step", "Browser-based"],
      },
      name: "Generate Thumbnails",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp"],
            extensions: [".jpg", ".jpeg", ".png", ".webp"],
            label: "JPEG, PNG, or WebP images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "resize",
          inputPorts: [],
          metadata: {},
          name: "Resize",
          outputPorts: [],
          parameters: {
            maintainAspect: true,
            quality: 80,
            width: 150,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-resize",
          version: "1.0.0",
        },
        {
          id: "convert",
          inputPorts: [],
          metadata: {},
          name: "Convert",
          outputPorts: [],
          parameters: {
            format: "webp",
            quality: 80,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "image-convert",
          version: "1.0.0",
        },
        {
          id: "rename",
          inputPorts: [],
          metadata: {},
          name: "Rename",
          outputPorts: [],
          parameters: {
            prefix: "thumb_",
          },
          position: {
            x: 750,
            y: 100,
          },
          type: "file-rename",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-generate-thumbnails",
            label: "Thumbnails",
            mode: "download",
            zip: true,
          },
          position: {
            x: 1000,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "compress-and-rename",
    name: "Compress & Rename",
    description:
      "Compress images and add a suffix so originals and compressed versions are distinguishable. Free, no signup.",
    category: "image",
    tags: ["Compress", "Rename", "Suffix", "Multi-step", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "compress",
        },
        {
          id: "e2",
          source: "compress",
          target: "rename",
        },
        {
          id: "e3",
          source: "rename",
          target: "output",
        },
      ],
      id: "compress-and-rename",
      inputPorts: [],
      metadata: {
        category: "image",
        description:
          "Compress images and add a suffix so originals and compressed versions are distinguishable. Free, no signup.",
        tags: ["Compress", "Rename", "Suffix", "Multi-step", "Browser-based"],
      },
      name: "Compress & Rename",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp"],
            extensions: [".jpg", ".jpeg", ".png", ".webp"],
            label: "JPEG, PNG, or WebP images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "compress",
          inputPorts: [],
          metadata: {},
          name: "Compress",
          outputPorts: [],
          parameters: {
            quality: 80,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-compress",
          version: "1.0.0",
        },
        {
          id: "rename",
          inputPorts: [],
          metadata: {},
          name: "Rename",
          outputPorts: [],
          parameters: {
            suffix: "-min",
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "file-rename",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-compress-and-rename",
            label: "Compressed & Renamed",
            mode: "download",
            zip: true,
          },
          position: {
            x: 750,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "standardize-csv",
    name: "Standardize CSV",
    description: "Clean up messy CSV data and rename column headers in one step. Free, no signup.",
    category: "spreadsheet",
    tags: ["CSV", "Clean", "Rename columns", "Multi-step", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "clean",
        },
        {
          id: "e2",
          source: "clean",
          target: "rename-columns",
        },
        {
          id: "e3",
          source: "rename-columns",
          target: "output",
        },
      ],
      id: "standardize-csv",
      inputPorts: [],
      metadata: {
        category: "spreadsheet",
        description:
          "Clean up messy CSV data and rename column headers in one step. Free, no signup.",
        tags: ["CSV", "Clean", "Rename columns", "Multi-step", "Browser-based"],
      },
      name: "Standardize CSV",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["text/csv"],
            extensions: [".csv"],
            label: "CSV files",
            mode: "file-upload",
            multiple: false,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "clean",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Clean",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            removeDuplicates: true,
            removeEmptyRows: true,
            trimWhitespace: true,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "spreadsheet-clean",
          version: "1.0.0",
        },
        {
          id: "rename-columns",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Rename",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            columns: {},
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "spreadsheet-rename",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-standardize-csv",
            label: "Standardized CSV",
            mode: "download",
            zip: false,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "strip-exif",
    name: "Strip EXIF",
    description:
      "Remove EXIF metadata from images instantly in your browser. No upload limits, no signup.",
    category: "image",
    tags: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "strip-exif-node",
        },
        {
          id: "e2",
          source: "strip-exif-node",
          target: "output",
        },
      ],
      id: "strip-exif",
      inputPorts: [],
      metadata: {
        category: "image",
        description:
          "Remove EXIF metadata from images instantly in your browser. No upload limits, no signup.",
        tags: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
      },
      name: "Strip EXIF",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp"],
            extensions: [".jpg", ".jpeg", ".png", ".webp"],
            label: "JPEG, PNG, or WebP images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "strip-exif-node",
          inputPorts: [],
          metadata: {},
          name: "Strip EXIF",
          outputPorts: [],
          parameters: {
            quality: 80,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-strip-exif",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-strip-exif",
            label: "Cleaned Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "watermark-images",
    name: "Watermark Images",
    description:
      "Add a logo or watermark to images. Position, size, and opacity are fully configurable. Runs in your browser — files never leave your machine.",
    category: "image",
    tags: [
      "PNG",
      "JPEG",
      "WebP",
      "Configurable position",
      "Adjustable opacity",
      "Browser-based",
    ] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "overlay",
        },
        {
          id: "e2",
          source: "overlay",
          target: "output",
        },
      ],
      id: "watermark-images",
      inputPorts: [],
      metadata: {
        category: "image",
        description:
          "Add a logo or watermark to images. Position, size, and opacity are fully configurable. Runs in your browser — files never leave your machine.",
        tags: [
          "PNG",
          "JPEG",
          "WebP",
          "Configurable position",
          "Adjustable opacity",
          "Browser-based",
        ],
      },
      name: "Watermark Images",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/jpeg", "image/png", "image/webp"],
            extensions: [".jpg", ".jpeg", ".png", ".webp"],
            label: "JPEG, PNG, or WebP images",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "overlay",
          inputPorts: [],
          metadata: {},
          name: "Overlay",
          outputPorts: [],
          parameters: {
            offsetX: 0,
            offsetY: 0,
            opacity: 80,
            overlay: "",
            position: "bottom-right",
            quality: 80,
            size: 25,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "image-overlay",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-watermark-images",
            label: "Watermarked Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "download-video",
    name: "Download Video",
    description:
      "Download video or audio from URLs using yt-dlp. Supports YouTube, m3u8/HLS streams, and hundreds of sites. CLI/desktop only.",
    category: "video",
    tags: ["YouTube", "m3u8/HLS", "MP4/WebM/MKV", "Audio extraction"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "download",
        },
        {
          id: "e2",
          source: "download",
          target: "output",
        },
      ],
      id: "download-video",
      inputPorts: [],
      metadata: {
        category: "video",
        description:
          "Download video or audio from URLs using yt-dlp. Supports YouTube, m3u8/HLS streams, and hundreds of sites. CLI/desktop only.",
        tags: ["YouTube", "m3u8/HLS", "MP4/WebM/MKV", "Audio extraction"],
      },
      name: "Download Video",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: [],
            extensions: [],
            label: "Video URL",
            mode: "url",
            multiple: true,
            placeholder: "https://youtube.com/watch?v=...",
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          fields: {
            audioCodec: {
              default: "m4a",
              label: "Audio Codec",
              options: [
                {
                  label: "M4A (AAC)",
                  value: "m4a",
                },
                {
                  label: "Opus",
                  value: "opus",
                },
                {
                  label: "MP3",
                  value: "mp3",
                },
              ],
              order: 3,
              type: "enum",
            },
            browser: {
              default: "",
              description:
                "Use cookies from a browser for authenticated content (Patreon, private videos)",
              label: "Browser Cookies",
              options: [
                {
                  label: "None",
                  value: "",
                },
                {
                  label: "Chrome",
                  value: "chrome",
                },
                {
                  label: "Firefox",
                  value: "firefox",
                },
                {
                  label: "Safari",
                  value: "safari",
                },
              ],
              order: 4,
              type: "enum",
            },
            format: {
              default: "mp4",
              description: "Video container format",
              label: "Output Format",
              options: [
                {
                  label: "MP4 (H.264)",
                  value: "mp4",
                },
                {
                  label: "WebM (VP9)",
                  value: "webm",
                },
                {
                  label: "MKV (Matroska)",
                  value: "mkv",
                },
              ],
              order: 1,
              type: "enum",
            },
            quality: {
              default: "",
              description: "Maximum video resolution (best available if unset)",
              label: "Quality Cap",
              options: [
                {
                  label: "Best available",
                  value: "",
                },
                {
                  label: "1080p",
                  value: "1080",
                },
                {
                  label: "720p",
                  value: "720",
                },
                {
                  label: "480p",
                  value: "480",
                },
              ],
              order: 5,
              type: "enum",
            },
            sponsorBlock: {
              default: "",
              description: "Remove sponsor segments from YouTube videos",
              label: "SponsorBlock",
              options: [
                {
                  label: "Keep all",
                  value: "",
                },
                {
                  label: "Remove sponsors",
                  value: "--sponsorblock-remove=default",
                },
              ],
              order: 8,
              type: "enum",
            },
            subtitles: {
              default: "",
              description: "Download and embed subtitles in the selected language",
              label: "Subtitles",
              options: [
                {
                  label: "None",
                  value: "",
                },
                {
                  label: "English",
                  value: "en",
                },
                {
                  label: "All languages",
                  value: "all",
                },
                {
                  label: "Common (EN/ES/FR/DE/JA)",
                  value: "en,es,fr,de,ja",
                },
              ],
              order: 6,
              type: "enum",
            },
            thumbnail: {
              default: "",
              description: "Embed the video thumbnail in the output file",
              label: "Embed Thumbnail",
              options: [
                {
                  label: "No",
                  value: "",
                },
                {
                  label: "Yes",
                  value: "--embed-thumbnail",
                },
              ],
              order: 7,
              type: "enum",
            },
            videoCodec: {
              default: "h264",
              label: "Video Codec",
              options: [
                {
                  label: "H.264",
                  value: "h264",
                },
                {
                  label: "VP9",
                  value: "vp9",
                },
                {
                  label: "AV1",
                  value: "av1",
                },
              ],
              order: 2,
              type: "enum",
            },
          },
          id: "download",
          inputPorts: [],
          metadata: {},
          name: "Download Video",
          outputPorts: [],
          parameters: {
            args: [
              "--no-playlist",
              "--no-warnings",
              "--newline",
              "--merge-output-format",
              "{{fields.format}}",
              "-S",
              "vcodec:{{fields.videoCodec}},acodec:{{fields.audioCodec}}",
              ["--cookies-from-browser", "{{fields.browser}}"],
              ["--write-subs", "--embed-subs", "--sub-langs", "{{fields.subtitles}}"],
              ["-S", "res:{{fields.quality}}"],
              "{{fields.thumbnail}}",
              "{{fields.sponsorBlock}}",
              "-o",
              "{{output_dir}}/%(title)s.%(ext)s",
            ],
            command: "yt-dlp",
            maxOutputSize: 10240,
            outputMode: "file",
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "shell-command",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-download-video",
            label: "Downloaded Video",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      requires: [
        {
          binary: "yt-dlp",
          homepage: "https://github.com/yt-dlp/yt-dlp",
          installHint: "brew install yt-dlp",
        },
        {
          binary: "ffmpeg",
          homepage: "https://ffmpeg.org",
          installHint: "brew install ffmpeg",
        },
      ],
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "svg-to-png",
    name: "SVG to PNG",
    description:
      "Convert SVG vector files to PNG raster images instantly in your browser. No upload limits, no signup.",
    category: "vector",
    tags: ["SVG", "PNG", "Vector to raster", "No upload", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "rasterize",
        },
        {
          id: "e2",
          source: "rasterize",
          target: "output",
        },
      ],
      id: "svg-to-png",
      inputPorts: [],
      metadata: {
        category: "vector",
        description:
          "Convert SVG vector files to PNG raster images instantly in your browser. No upload limits, no signup.",
        tags: ["SVG", "PNG", "Vector to raster", "No upload", "Browser-based"],
      },
      name: "SVG to PNG",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/svg+xml"],
            extensions: [".svg"],
            label: "SVG files",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "rasterize",
          inputPorts: [],
          metadata: {},
          name: "Rasterize",
          outputPorts: [],
          parameters: {
            format: "png",
            quality: 80,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "vector-rasterize",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-svg-to-png",
            label: "PNG Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "svg-to-jpeg",
    name: "SVG to JPEG",
    description:
      "Convert SVG vector files to JPEG raster images instantly in your browser. No upload limits, no signup.",
    category: "vector",
    tags: ["SVG", "JPEG", "Vector to raster", "No upload", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "rasterize",
        },
        {
          id: "e2",
          source: "rasterize",
          target: "output",
        },
      ],
      id: "svg-to-jpeg",
      inputPorts: [],
      metadata: {
        category: "vector",
        description:
          "Convert SVG vector files to JPEG raster images instantly in your browser. No upload limits, no signup.",
        tags: ["SVG", "JPEG", "Vector to raster", "No upload", "Browser-based"],
      },
      name: "SVG to JPEG",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/svg+xml"],
            extensions: [".svg"],
            label: "SVG files",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "rasterize",
          inputPorts: [],
          metadata: {},
          name: "Rasterize",
          outputPorts: [],
          parameters: {
            format: "jpeg",
            quality: 80,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "vector-rasterize",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-svg-to-jpeg",
            label: "JPEG Images",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "optimize-svg",
    name: "Optimize SVG",
    description:
      "Optimize SVG files by removing editor metadata, comments, and unnecessary elements. Runs entirely in your browser.",
    category: "vector",
    tags: ["SVG", "Optimize", "Minify", "Clean", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "optimize",
        },
        {
          id: "e2",
          source: "optimize",
          target: "output",
        },
      ],
      id: "optimize-svg",
      inputPorts: [],
      metadata: {
        category: "vector",
        description:
          "Optimize SVG files by removing editor metadata, comments, and unnecessary elements. Runs entirely in your browser.",
        tags: ["SVG", "Optimize", "Minify", "Clean", "Browser-based"],
      },
      name: "Optimize SVG",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["image/svg+xml"],
            extensions: [".svg"],
            label: "SVG files",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "optimize",
          inputPorts: [],
          metadata: {},
          name: "Optimize",
          outputPorts: [],
          parameters: {
            collapseGroups: true,
            minify: true,
            precision: 3,
            removeComments: true,
            removeMetadata: true,
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "vector-optimize",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-optimize-svg",
            label: "Optimized SVGs",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "number-files",
    name: "Number Files",
    description: "Add sequential numbers to filenames. Free, no signup required.",
    category: "file",
    tags: ["Sequential numbering", "Batch rename", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "number-file",
        },
        {
          id: "e2",
          source: "number-file",
          target: "output",
        },
      ],
      id: "number-files",
      inputPorts: [],
      metadata: {
        category: "file",
        description: "Add sequential numbers to filenames. Free, no signup required.",
        tags: ["Sequential numbering", "Batch rename", "Browser-based"],
      },
      name: "Number Files",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["*/*"],
            extensions: [],
            label: "any files",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "number-file",
          inputPorts: [],
          metadata: {},
          name: "Number",
          outputPorts: [],
          parameters: {
            counter_pad: 3,
            counter_start: 1,
            pattern: "{{counter}}-{{name}}.{{ext}}",
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "file-rename",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-number-files",
            label: "Numbered Files",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "sanitize-filenames",
    name: "Sanitize Filenames",
    description: "Clean filenames for web-safe or cross-platform use. Free, no signup required.",
    category: "file",
    tags: ["Filename cleanup", "Web-safe", "Browser-based"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "sanitize-file",
        },
        {
          id: "e2",
          source: "sanitize-file",
          target: "output",
        },
      ],
      id: "sanitize-filenames",
      inputPorts: [],
      metadata: {
        category: "file",
        description:
          "Clean filenames for web-safe or cross-platform use. Free, no signup required.",
        tags: ["Filename cleanup", "Web-safe", "Browser-based"],
      },
      name: "Sanitize Filenames",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            accept: ["*/*"],
            extensions: [],
            label: "any files",
            mode: "file-upload",
            multiple: true,
          },
          position: {
            x: 0,
            y: 100,
          },
          type: "input",
          version: "1.0.0",
        },
        {
          id: "sanitize-file",
          inputPorts: [],
          metadata: {},
          name: "Sanitize",
          outputPorts: [],
          parameters: {
            sanitize: "slugify",
            separator: "-",
          },
          position: {
            x: 250,
            y: 100,
          },
          type: "file-rename",
          version: "1.0.0",
        },
        {
          id: "output",
          inputPorts: [
            {
              id: "in-1",
              name: "files",
            },
          ],
          metadata: {},
          name: "Output",
          outputPorts: [],
          parameters: {
            autoDownload: true,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-sanitize-filenames",
            label: "Sanitized Files",
            mode: "download",
            zip: true,
          },
          position: {
            x: 500,
            y: 100,
          },
          type: "output",
          version: "1.0.0",
        },
      ],
      outputPorts: [],
      parameters: {},
      position: {
        x: 0,
        y: 0,
      },
      settings: {
        iteration: "auto",
      },
      type: "group",
      version: "1.0.0",
    },
  },
] as const;
