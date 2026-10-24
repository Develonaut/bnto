/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { Definition } from "../definition";

/** An operating system a recipe can run on (what `{{ctx.platform}}` reads). */
export type RecipeOs = "macos" | "linux" | "windows";

/** A generated recipe from the engine's built-in catalog. */
export interface GeneratedRecipe {
  readonly slug: string;
  readonly name: string;
  readonly description: string;
  readonly category: string;
  readonly tags: readonly string[];
  /** The OSes it has steps for, worked out by the engine from its platform guards. */
  readonly os: readonly RecipeOs[];
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            enlarge: false,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            enlarge: false,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
      "Download video or audio from URLs using yt-dlp. Supports YouTube, m3u8/HLS streams, and hundreds of sites. CLI only.",
    category: "video",
    tags: ["YouTube", "m3u8/HLS", "MP4/WebM/MKV", "Audio extraction"] as const,
    os: ["macos", "linux", "windows"] as const,
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
          "Download video or audio from URLs using yt-dlp. Supports YouTube, m3u8/HLS streams, and hundreds of sites. CLI only.",
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
          type: "shell-run",
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
            mode: "write",
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
          installHint: "see https://github.com/yt-dlp/yt-dlp#installation",
          installHints: {
            linux: "pipx install yt-dlp",
            macos: "brew install yt-dlp",
            windows: "winget install yt-dlp.yt-dlp",
          },
        },
        {
          binary: "ffmpeg",
          homepage: "https://ffmpeg.org",
          installHint: "see https://ffmpeg.org/download.html",
          installHints: {
            linux: "sudo apt install ffmpeg",
            macos: "brew install ffmpeg",
            windows: "winget install Gyan.FFmpeg",
          },
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            counterPad: 3,
            counterStart: 1,
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
            mode: "write",
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
    os: ["macos", "linux", "windows"] as const,
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
            mode: "write",
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
    slug: "flatten-folders",
    name: "Flatten Folders",
    description:
      "Collect all files from nested subdirectories into a single flat output folder. CLI only.",
    category: "file",
    tags: ["Organize", "Flatten", "Folders", "Cleanup"] as const,
    os: ["macos", "linux", "windows"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "collect",
        },
        {
          id: "e2",
          source: "collect",
          target: "output",
        },
      ],
      id: "flatten-folders",
      inputPorts: [],
      metadata: {
        category: "file",
        description:
          "Collect all files from nested subdirectories into a single flat output folder. CLI only.",
        tags: ["Organize", "Flatten", "Folders", "Cleanup"],
      },
      name: "Flatten Folders",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "Source Directory",
          outputPorts: [
            {
              id: "out-1",
              name: "directory",
            },
          ],
          parameters: {
            accept: ["*/*"],
            extensions: [],
            label: "directory to flatten",
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
          id: "collect",
          inputPorts: [],
          metadata: {},
          name: "Collect All Files",
          outputPorts: [],
          parameters: {
            flatten: true,
            pattern: "*",
            recursive: true,
          },
          position: {
            x: 200,
            y: 100,
          },
          type: "file-collect",
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
          name: "Flattened Files",
          outputPorts: [],
          parameters: {
            autoDownload: false,
            directory: "{{ctx.paths.output_dir}}/{{ctx.date}}-flatten-folders",
            label: "Flattened Files",
            mode: "write",
            zip: false,
          },
          position: {
            x: 400,
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
        iteration: "explicit",
      },
      type: "group",
      version: "1.0.0",
    },
  },
  {
    slug: "keep-awake",
    name: "Keep Awake",
    description:
      "Keep your computer awake for a set time or until you stop it. By default it also nudges the cursor once you go idle, so chat apps don't mark you Away. Start a session, come back later and run Stop. CLI only.",
    category: "system",
    tags: ["macOS", "Linux", "Windows", "Sleep prevention", "Activity simulation"] as const,
    os: ["macos", "linux", "windows"] as const,
    definition: {
      edges: [
        {
          id: "e1",
          source: "input",
          target: "session",
        },
        {
          id: "e2",
          source: "session",
          target: "output",
        },
      ],
      id: "keep-awake",
      inputPorts: [],
      metadata: {
        category: "system",
        description:
          "Keep your computer awake for a set time or until you stop it. By default it also nudges the cursor once you go idle, so chat apps don't mark you Away. Start a session, come back later and run Stop. CLI only.",
        tags: ["macOS", "Linux", "Windows", "Sleep prevention", "Activity simulation"],
      },
      name: "Keep Awake",
      nodes: [
        {
          id: "input",
          inputPorts: [],
          metadata: {},
          name: "No Input",
          outputPorts: [
            {
              id: "out-1",
              name: "files",
            },
          ],
          parameters: {
            mode: "none",
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
            action: {
              default: "start",
              description:
                "Start a new session, stop the running one, or check what's currently active.",
              label: "Action",
              options: [
                {
                  label: "Start a session",
                  value: "start",
                },
                {
                  label: "Stop the session",
                  value: "stop",
                },
                {
                  label: "Check status",
                  value: "status",
                },
              ],
              order: 1,
              type: "enum",
            },
            display: {
              default: "keep-on",
              description:
                "Keep the display awake so it doesn't sleep. Allow sleep if you only need background tasks to keep running. On Linux, GNOME needs gnome-session-inhibit for this.",
              label: "Display",
              options: [
                {
                  label: "Keep display awake (recommended)",
                  value: "keep-on",
                },
                {
                  label: "Allow display sleep",
                  value: "allow-sleep",
                },
              ],
              order: 3,
              type: "enum",
            },
            duration: {
              default: "3600",
              description: "How long to stay awake. Ignored for Stop and Check status.",
              label: "Session Length",
              options: [
                {
                  label: "30 minutes",
                  value: "1800",
                },
                {
                  label: "1 hour",
                  value: "3600",
                },
                {
                  label: "2 hours",
                  value: "7200",
                },
                {
                  label: "3 hours",
                  value: "10800",
                },
                {
                  label: "4 hours",
                  value: "14400",
                },
                {
                  label: "6 hours",
                  value: "21600",
                },
                {
                  label: "8 hours",
                  value: "28800",
                },
                {
                  label: "10 hours",
                  value: "36000",
                },
                {
                  label: "12 hours",
                  value: "43200",
                },
                {
                  label: "Indefinitely (until stopped)",
                  value: "inf",
                },
              ],
              order: 2,
              type: "enum",
            },
            jiggle: {
              default: "120",
              description:
                "Keeping the computer awake doesn't reset the idle timer, so chat apps still mark you Away. This nudges the cursor 1px once you've been idle this long. macOS needs Accessibility permission for your terminal app; Linux needs an X11 session with xdotool and xprintidle; Windows needs nothing more.",
              label: "Activity Simulation",
              options: [
                {
                  label: "After 2 minutes idle (recommended)",
                  value: "120",
                },
                {
                  label: "After 1 minute idle",
                  value: "60",
                },
                {
                  label: "After 4 minutes idle",
                  value: "240",
                },
                {
                  label: "Off — sleep prevention only",
                  value: "off",
                },
              ],
              order: 4,
              type: "enum",
            },
          },
          id: "session",
          inputPorts: [],
          metadata: {},
          name: "Keep-Awake Session",
          nodes: [
            {
              id: "macos",
              inputPorts: [],
              metadata: {},
              name: "macOS",
              nodes: [
                {
                  id: "caffeinate",
                  inputPorts: [],
                  metadata: {},
                  name: "Caffeinate Session",
                  outputPorts: [],
                  parameters: {
                    args: [
                      "{{ctx.paths.home_dir}}",
                      "{{fields.action}}",
                      "{{fields.duration}}",
                      "{{fields.display}}",
                      "{{fields.jiggle}}",
                    ],
                    outputMode: "stdout",
                    script:
                      '# keep-awake: a background caffeinate session you start, check and stop.\n# $1 bnto home, $2 action, $3 duration (seconds | inf),\n# $4 display (keep-on | allow-sleep), $5 activity simulation (idle seconds | off).\ndie() { echo "keep-awake: $*" >&2; exit 2; }\nhome=$1 action=$2 duration=$3 display=$4 jiggle=$5\n[ -n "$home" ] || die "bnto home is unknown"\ncase $action in start | stop | status) ;; *) die "unknown action \'$action\'" ;; esac\n# A leading zero would make sh\'s arithmetic read the duration as octal.\ncase $duration in inf) ;; \'\' | 0* | *[!0-9]*) die "bad duration \'$duration\'" ;; esac\ncase $display in keep-on | allow-sleep) ;; *) die "bad display setting \'$display\'" ;; esac\ncase $jiggle in off) ;; \'\' | *[!0-9]*) die "bad idle threshold \'$jiggle\'" ;; esac\n\nstate_dir="$home/state"\nstate="$state_dir/keep-awake.json"\n\n# JXA helper, one entry point per mode, values via its own argv:\n#   trusted         -- does the terminal have Accessibility permission?\n#   idle            -- seconds since the last keyboard/mouse input\n#   loop SECS UNTIL -- nudge the cursor 1px whenever idle >= SECS, until\n#                      UNTIL (epoch seconds, 0 = forever)\n# A posted mouse move resets the system idle timer; caffeinate alone does\n# not, which is why chat apps still go Away during a plain caffeinate session.\n# shellcheck disable=SC2016 # JavaScript, not shell: nothing should expand.\nJXA=\'ObjC.import("CoreGraphics"); ObjC.import("ApplicationServices");\nfunction idle() { return $.CGEventSourceSecondsSinceLastEventType($.kCGEventSourceStateHIDSystemState, 0xFFFFFFFF); }\nfunction move(x, y) { $.CGEventPost($.kCGHIDEventTap, $.CGEventCreateMouseEvent($(), $.kCGEventMouseMoved, {x: x, y: y}, 0)); }\nfunction nudge() { var p = $.CGEventGetLocation($.CGEventCreate($())); move(p.x + 1, p.y); delay(0.15); move(p.x, p.y); }\nfunction run(argv) {\n  if (argv[0] === "trusted") return String($.AXIsProcessTrusted());\n  if (argv[0] === "idle") return String(Math.round(idle()));\n  var threshold = Number(argv[1]), ends = Number(argv[2]);\n  while (!ends || Date.now() / 1000 < ends) { delay(15); if (idle() >= threshold) nudge(); }\n}\'\njxa() { /usr/bin/osascript -l JavaScript -e "$JXA" "$@"; }\n\n# One field from the state file this script writes below.\nfield() { [ -f "$state" ] && sed -n "s/.*\\"$1\\": *\\"*\\([^,\\"}]*\\).*/\\1/p" "$state"; }\n\n# False once a timed session\'s end has passed: its caffeinate has exited by\n# then, so whatever runs at that pid now is another program\'s.\ncurrent_state() {\n  end=$(field until)\n  end=${end%.*}\n  case $end in \'\') return 0 ;; *[!0-9]*) return 1 ;; esac\n  [ "$end" -gt "$(date +%s)" ]\n}\n\n# Print pid $1 if it still runs the command this script started as $2\n# (session | nudger), so a stale pid now used by another program doesn\'t count.\nours() {\n  case $1 in \'\' | *[!0-9]*) return 1 ;; esac\n  current_state || return 1\n  args=$(ps -ww -p "$1" -o args= 2>/dev/null) || return 1\n  case $2:$args in\n    session:"/usr/bin/caffeinate -i -s -t "* | session:"/usr/bin/caffeinate -du -i -s -t "*) echo "$1" ;;\n    nudger:"/usr/bin/osascript -l JavaScript -e "*" loop "*) echo "$1" ;;\n    *) return 1 ;;\n  esac\n}\n\nsession_pid() { ours "$(field pid)" session; }\njiggle_pid() { ours "$(field jigglePid)" nudger; }\n\n# End the session and its nudger, each only if it\'s still ours: either can\n# outlive the other (say, after `killall caffeinate`).\nend_session() {\n  s=$(session_pid) j=$(jiggle_pid)\n  if [ -n "$s" ]; then kill "$s" 2>/dev/null; fi\n  if [ -n "$j" ]; then kill "$j" 2>/dev/null; fi\n  rm -f "$state"\n}\n\nremaining() {\n  ends=$(field until)\n  if [ -z "$ends" ]; then echo "running indefinitely"; return; fi\n  # Round up, so a fresh 30-minute session reads 30m, not 29m.\n  m=$(( (${ends%.*} - $(date +%s) + 59) / 60 ))\n  if [ "$m" -lt 0 ]; then m=0; fi\n  printf \'%dh %02dm remaining of %s\\n\' $((m / 60)) $((m % 60)) "$(field label)"\n}\n\nplural() { if [ "$1" = 1 ]; then echo "1 $2"; else echo "$1 ${2}s"; fi; }\n\nlabel_for() {\n  if [ "$1" = inf ]; then echo "indefinitely"; return; fi\n  h=$(($1 / 3600)) m=$(($1 % 3600 / 60))\n  if [ "$h" = 0 ]; then\n    plural "$m" minute\n  elif [ "$m" = 0 ]; then\n    plural "$h" hour\n  else\n    echo "$(plural "$h" hour) $(plural "$m" minute)"\n  fi\n}\n\ncurrent=$(session_pid)\nnudger=$(jiggle_pid)\n\nif [ "$action" = stop ]; then\n  if [ -n "$current$nudger" ]; then\n    end_session\n    echo "Session ended -- your Mac can sleep normally again."\n  else\n    rm -f "$state"\n    echo "No active session -- nothing to stop."\n  fi\n  exit 0\nfi\n\nif [ "$action" = status ]; then\n  if [ -z "$current" ]; then\n    echo "No active session."\n    if [ -n "$nudger" ]; then echo "Activity simulation is still running -- run Stop to end it."; fi\n    exit 0\n  fi\n  echo "Session active -- $(remaining)."\n  if [ -z "$(field jigglePid)" ]; then\n    sim=off\n  elif [ -n "$nudger" ]; then\n    sim=on\n  else\n    sim="stopped -- start a new session to restart it"\n  fi\n  echo "Activity simulation: $sim"\n  echo "Idle for $(jxa idle)s right now."\n  exit 0\nfi\n\n# --- start ---\nmkdir -p "$state_dir" || die "can\'t create $state_dir"\nreplaced=""\nif [ -n "$current$nudger" ]; then replaced="Replaced previous session -- "; fi\nend_session\n\nlabel=$(label_for "$duration")\nif [ "$duration" = inf ]; then\n  secs=315360000 ends=0\nelse\n  secs=$duration ends=$(( $(date +%s) + duration ))\nfi\n\nset -- -i -s -t "$secs"\n# -d keeps the display awake; -u declares the user active, waking it if off.\nif [ "$display" = keep-on ]; then set -- -du "$@"; fi\n# Detached with stdio closed, so bnto sees the step finish right away.\nnohup /usr/bin/caffeinate "$@" </dev/null >/dev/null 2>&1 &\nextra=", \\"pid\\": $!"\nif [ "$ends" != 0 ]; then extra="$extra, \\"until\\": $ends"; fi\n\nwarning=""\nif [ "$jiggle" = off ]; then\n  sim=off\nelif [ "$(jxa trusted)" = true ]; then\n  nohup /usr/bin/osascript -l JavaScript -e "$JXA" loop "$jiggle" "$ends" </dev/null >/dev/null 2>&1 &\n  extra="$extra, \\"jigglePid\\": $!"\n  sim="on (nudges the cursor after ${jiggle}s idle)"\nelse\n  sim=off\n  warning="\nWARNING: activity simulation needs Accessibility permission -- chat apps will still mark you Away.\n  Grant it to your terminal app: System Settings > Privacy & Security > Accessibility"\nfi\n\nprintf \'{"label": "%s"%s}\\n\' "$label" "$extra" >"$state"\n\nif [ "$duration" = inf ]; then what="indefinitely (until you stop it)"; else what="for $label"; fi\necho "${replaced}Session started: staying awake $what."\necho "Activity simulation: $sim"\necho "Run keep-awake again with Action: Stop to end it early.$warning"\n',
                    shell: "sh",
                    timeout: 60,
                  },
                  position: {
                    x: 0,
                    y: 0,
                  },
                  type: "shell-run",
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
                  binary: "caffeinate",
                  homepage: "https://ss64.com/mac/caffeinate.html",
                  installHints: {
                    macos: "built into macOS — nothing to install",
                  },
                },
                {
                  binary: "osascript",
                  homepage: "https://ss64.com/mac/osascript.html",
                  installHints: {
                    macos:
                      "built into macOS — activity simulation needs Accessibility permission for your terminal app",
                  },
                },
              ],
              type: "group",
              version: "1.0.0",
              when: {
                expected: "macos",
                op: "equals",
                value: "{{ctx.platform}}",
              },
            },
            {
              id: "linux",
              inputPorts: [],
              metadata: {},
              name: "Linux",
              nodes: [
                {
                  id: "inhibit",
                  inputPorts: [],
                  metadata: {},
                  name: "systemd-inhibit Session",
                  outputPorts: [],
                  parameters: {
                    args: [
                      "{{ctx.paths.home_dir}}",
                      "{{fields.action}}",
                      "{{fields.duration}}",
                      "{{fields.display}}",
                      "{{fields.jiggle}}",
                    ],
                    env: {
                      DBUS_SESSION_BUS_ADDRESS: "{{env.DBUS_SESSION_BUS_ADDRESS}}",
                      DISPLAY: "{{env.DISPLAY}}",
                      WAYLAND_DISPLAY: "{{env.WAYLAND_DISPLAY}}",
                      XAUTHORITY: "{{env.XAUTHORITY}}",
                      XDG_RUNTIME_DIR: "{{env.XDG_RUNTIME_DIR}}",
                    },
                    outputMode: "stdout",
                    script:
                      '# keep-awake (Linux): a background systemd-inhibit session you start, check and stop.\n# $1 bnto home, $2 action, $3 duration (seconds | inf),\n# $4 display (keep-on | allow-sleep), $5 activity simulation (idle seconds | off).\n# The desktop session\'s DISPLAY, XAUTHORITY, WAYLAND_DISPLAY,\n# DBUS_SESSION_BUS_ADDRESS and XDG_RUNTIME_DIR arrive through the step\'s env.\ndie() { echo "keep-awake: $*" >&2; exit 2; }\nhome=$1 action=$2 duration=$3 display=$4 jiggle=$5\n[ -n "$home" ] || die "bnto home is unknown"\ncase $action in start | stop | status) ;; *) die "unknown action \'$action\'" ;; esac\n# A leading zero would make sh\'s arithmetic read the duration as octal.\ncase $duration in inf) ;; \'\' | 0* | *[!0-9]*) die "bad duration \'$duration\'" ;; esac\ncase $display in keep-on | allow-sleep) ;; *) die "bad display setting \'$display\'" ;; esac\ncase $jiggle in off) ;; \'\' | *[!0-9]*) die "bad idle threshold \'$jiggle\'" ;; esac\n\n# An empty variable from the step\'s env means the desktop didn\'t set it.\n[ -n "${DISPLAY-}" ] || unset DISPLAY\n[ -n "${XAUTHORITY-}" ] || unset XAUTHORITY\n[ -n "${WAYLAND_DISPLAY-}" ] || unset WAYLAND_DISPLAY\n[ -n "${DBUS_SESSION_BUS_ADDRESS-}" ] || unset DBUS_SESSION_BUS_ADDRESS\n[ -n "${XDG_RUNTIME_DIR-}" ] || unset XDG_RUNTIME_DIR\n\nstate_dir="$home/state"\nstate="$state_dir/keep-awake.json"\n\n# A program by its usual absolute path, never a PATH lookup; empty if absent.\nbin() { for dir in /usr/bin /bin; do if [ -x "$dir/$1" ]; then echo "$dir/$1"; return; fi; done; }\ninhibit=$(bin systemd-inhibit)\ngnome=$(bin gnome-session-inhibit)\nxdotool=$(bin xdotool)\nxprintidle=$(bin xprintidle)\nsleeper=$(bin sleep)\n\n# The cursor nudger, run as its own sh: $1 idle threshold, $2 end (epoch\n# seconds, 0 = forever), $3 xdotool, $4 xprintidle. An XTEST mouse move\n# resets the X idle timer, which a sleep inhibitor alone does not.\n# shellcheck disable=SC2016 # Expanded by the nudger\'s own sh, not this one.\nNUDGE=\'while [ "$2" = 0 ] || [ "$(date +%s)" -lt "$2" ]; do sleep 15; ms=$("$4" 2>/dev/null); case $ms in "" | *[!0-9]*) continue ;; esac; if [ $((ms / 1000)) -ge "$1" ]; then "$3" mousemove_relative -- 1 0; sleep 0.15; "$3" mousemove_relative -- -1 0; fi; done\'\n\n# One field from the state file this script writes below.\nfield() { [ -f "$state" ] && sed -n "s/.*\\"$1\\": *\\"*\\([^,\\"}]*\\).*/\\1/p" "$state"; }\n\n# False once a timed session\'s end has passed: its inhibitors have exited\n# by then, so whatever runs at that pid now is another program\'s.\ncurrent_state() {\n  end=$(field until)\n  end=${end%.*}\n  case $end in \'\') return 0 ;; *[!0-9]*) return 1 ;; esac\n  [ "$end" -gt "$(date +%s)" ]\n}\n\n# Print pid $1 if it still runs the command this script started as $2\n# (session | display | nudger), so a stale pid now used by another program\n# doesn\'t count.\nours() {\n  case $1 in \'\' | *[!0-9]*) return 1 ;; esac\n  current_state || return 1\n  args=$(ps -ww -p "$1" -o args= 2>/dev/null) || return 1\n  case $2:$args in\n    session:*"systemd-inhibit --who=keep-awake "*) echo "$1" ;;\n    display:*"gnome-session-inhibit --app-id keep-awake "*) echo "$1" ;;\n    nudger:"/bin/sh -c "*" keep-awake-nudger "*) echo "$1" ;;\n    *) return 1 ;;\n  esac\n}\n\nsession_pid() { ours "$(field pid)" session; }\ndisplay_pid() { ours "$(field displayPid)" display; }\njiggle_pid() { ours "$(field jigglePid)" nudger; }\n\n# End the session, its display inhibitor and its nudger, each only if it\'s\n# still ours: any of them can outlive the others.\nend_session() {\n  for pid in $(session_pid) $(display_pid) $(jiggle_pid); do kill "$pid" 2>/dev/null; done\n  rm -f "$state"\n}\n\nremaining() {\n  ends=$(field until)\n  if [ -z "$ends" ]; then echo "running indefinitely"; return; fi\n  # Round up, so a fresh 30-minute session reads 30m, not 29m.\n  m=$(( (${ends%.*} - $(date +%s) + 59) / 60 ))\n  if [ "$m" -lt 0 ]; then m=0; fi\n  printf \'%dh %02dm remaining of %s\\n\' $((m / 60)) $((m % 60)) "$(field label)"\n}\n\nplural() { if [ "$1" = 1 ]; then echo "1 $2"; else echo "$1 ${2}s"; fi; }\n\nlabel_for() {\n  if [ "$1" = inf ]; then echo "indefinitely"; return; fi\n  h=$(($1 / 3600)) m=$(($1 % 3600 / 60))\n  if [ "$h" = 0 ]; then\n    plural "$m" minute\n  elif [ "$m" = 0 ]; then\n    plural "$h" hour\n  else\n    echo "$(plural "$h" hour) $(plural "$m" minute)"\n  fi\n}\n\n# Whether the cursor nudger can work here: X11 (not Wayland, where X input\n# doesn\'t reach the compositor), with xdotool and xprintidle installed.\ncan_nudge() { [ -n "${DISPLAY-}" ] && [ -z "${WAYLAND_DISPLAY-}" ] && [ -n "$xdotool" ] && [ -n "$xprintidle" ]; }\n\n# Start "$@" detached and wait a moment: $launched is its pid if it\'s\n# still running, else empty, with what it said in $failed.\nlaunch() {\n  err="$state_dir/keep-awake.err"\n  nohup "$@" </dev/null >/dev/null 2>"$err" &\n  launched=$! failed=""\n  "$sleeper" 1\n  if ! kill -0 "$launched" 2>/dev/null; then launched="" failed=$(head -c 300 "$err"); fi\n  rm -f "$err"\n}\n\n# Seconds since the last keyboard or mouse input on X11, or nothing.\nidle_secs() {\n  ms=$("$xprintidle" 2>/dev/null)\n  case $ms in \'\' | *[!0-9]*) ;; *) echo $((ms / 1000)) ;; esac\n}\n\ncurrent=$(session_pid)\nshown=$(display_pid)\nnudger=$(jiggle_pid)\n\nif [ "$action" = stop ]; then\n  if [ -n "$current$shown$nudger" ]; then\n    end_session\n    echo "Session ended -- this computer can sleep normally again."\n  else\n    rm -f "$state"\n    echo "No active session -- nothing to stop."\n  fi\n  exit 0\nfi\n\nif [ "$action" = status ]; then\n  if [ -z "$current" ]; then\n    echo "No active session."\n    if [ -n "$shown$nudger" ]; then echo "Part of an old session is still running -- run Stop to end it."; fi\n    exit 0\n  fi\n  echo "Session active -- $(remaining)."\n  if [ -z "$(field jigglePid)" ]; then\n    sim=off\n  elif [ -n "$nudger" ]; then\n    sim=on\n  else\n    sim="stopped -- start a new session to restart it"\n  fi\n  echo "Activity simulation: $sim"\n  idle=""\n  if can_nudge; then idle=$(idle_secs); fi\n  if [ -n "$idle" ]; then echo "Idle for ${idle}s right now."; fi\n  exit 0\nfi\n\n# --- start ---\n[ -n "$inhibit" ] || die "systemd-inhibit isn\'t installed -- keep-awake on Linux needs systemd"\nmkdir -p "$state_dir" || die "can\'t create $state_dir"\nreplaced=""\nif [ -n "$current$shown$nudger" ]; then replaced="Replaced previous session -- "; fi\nend_session\n\nlabel=$(label_for "$duration")\nif [ "$duration" = inf ]; then\n  secs=315360000 ends=0\nelse\n  secs=$duration ends=$(( $(date +%s) + duration ))\nfi\n\n# sleep blocks suspend; idle also blocks idle actions where the desktop\n# honors logind\'s idle inhibitor (KDE Plasma does, GNOME doesn\'t).\nwhat="sleep"\nif [ "$display" = keep-on ]; then what="sleep:idle"; fi\nlaunch "$inhibit" --who=keep-awake --why="bnto keep-awake session" --what="$what" --mode=block "$sleeper" "$secs"\n# polkit can refuse the idle lock but grant the sleep one: keep the session.\nif [ -z "$launched" ] && [ "$what" = "sleep:idle" ]; then\n  what="sleep"\n  launch "$inhibit" --who=keep-awake --why="bnto keep-awake session" --what="$what" --mode=block "$sleeper" "$secs"\nfi\nif [ -z "$launched" ]; then\n  die "systemd refused to keep this computer awake: ${failed:-no reason given}\n  Outside a local desktop session (an SSH login, a service), blocking sleep needs an administrator."\nfi\nextra=", \\"pid\\": $launched"\nif [ "$ends" != 0 ]; then extra="$extra, \\"until\\": $ends"; fi\n\n# GNOME blanks and locks the screen unless its own session inhibitor holds.\nscreen="allowed to sleep"\nif [ "$display" = keep-on ]; then\n  screen="kept on where the desktop honors systemd\'s idle inhibitor (KDE Plasma does; on GNOME install gnome-session-inhibit)"\n  if [ "$what" = "sleep" ]; then screen="may still sleep -- systemd refused its idle inhibitor"; fi\n  if [ -n "$gnome" ] && [ -n "${DBUS_SESSION_BUS_ADDRESS-}${XDG_RUNTIME_DIR-}" ]; then\n    launch "$gnome" --app-id keep-awake --inhibit idle:suspend --reason "bnto keep-awake session" "$sleeper" "$secs"\n    if [ -n "$launched" ]; then\n      extra="$extra, \\"displayPid\\": $launched"\n      screen="kept on"\n    fi\n  fi\nfi\n\nnotice=""\nif [ "$jiggle" = off ]; then\n  sim=off\nelif can_nudge; then\n  nohup /bin/sh -c "$NUDGE" keep-awake-nudger "$jiggle" "$ends" "$xdotool" "$xprintidle" </dev/null >/dev/null 2>&1 &\n  extra="$extra, \\"jigglePid\\": $!"\n  sim="on (nudges the cursor after ${jiggle}s idle)"\nelse\n  sim=off\n  notice="\nNOTICE: activity simulation needs an X11 desktop session with xdotool and xprintidle installed\n  (Wayland doesn\'t let one app move the cursor for another) -- chat apps may still mark you Away."\nfi\n\nprintf \'{"label": "%s"%s}\\n\' "$label" "$extra" >"$state"\n\nif [ "$duration" = inf ]; then span="indefinitely (until you stop it)"; else span="for $label"; fi\necho "${replaced}Session started: staying awake $span."\necho "Display: $screen"\necho "Activity simulation: $sim"\necho "Run keep-awake again with Action: Stop to end it early.$notice"\n',
                    shell: "sh",
                    timeout: 60,
                  },
                  position: {
                    x: 0,
                    y: 0,
                  },
                  type: "shell-run",
                  version: "1.0.0",
                },
              ],
              outputPorts: [],
              parameters: {},
              position: {
                x: 0,
                y: 150,
              },
              requires: [
                {
                  binary: "systemd-inhibit",
                  homepage:
                    "https://www.freedesktop.org/software/systemd/man/latest/systemd-inhibit.html",
                  installHints: {
                    linux: "part of systemd, which most Linux distributions ship with",
                  },
                },
              ],
              type: "group",
              version: "1.0.0",
              when: {
                expected: "linux",
                op: "equals",
                value: "{{ctx.platform}}",
              },
            },
            {
              id: "windows",
              inputPorts: [],
              metadata: {},
              name: "Windows",
              nodes: [
                {
                  id: "wake",
                  inputPorts: [],
                  metadata: {},
                  name: "Windows Keep-Awake Session",
                  outputPorts: [],
                  parameters: {
                    args: [
                      "{{ctx.paths.home_dir}}",
                      "{{fields.action}}",
                      "{{fields.duration}}",
                      "{{fields.display}}",
                      "{{fields.jiggle}}",
                    ],
                    outputMode: "stdout",
                    script:
                      "# keep-awake (Windows): a background session you start, check and stop.\n# $env:BNTO_ARG_1 bnto home, _2 action, _3 duration (seconds | inf),\n# _4 display (keep-on | allow-sleep), _5 activity simulation (idle seconds | off).\n$ErrorActionPreference = 'Stop'\nfunction Exit-Failure([string]$Message) {\n    [Console]::Error.WriteLine(\"keep-awake: $Message\")\n    exit 2\n}\n$homeDir = [string]$env:BNTO_ARG_1\n$action = [string]$env:BNTO_ARG_2\n$duration = [string]$env:BNTO_ARG_3\n$display = [string]$env:BNTO_ARG_4\n$jiggle = [string]$env:BNTO_ARG_5\nif (-not $homeDir) { Exit-Failure 'bnto home is unknown' }\nif ($action -notin 'start', 'stop', 'status') { Exit-Failure \"unknown action '$action'\" }\nif ($duration -ne 'inf' -and $duration -notmatch '^[1-9][0-9]*$') { Exit-Failure \"bad duration '$duration'\" }\nif ($display -notin 'keep-on', 'allow-sleep') { Exit-Failure \"bad display setting '$display'\" }\nif ($jiggle -ne 'off' -and $jiggle -notmatch '^[1-9][0-9]*$') { Exit-Failure \"bad idle threshold '$jiggle'\" }\n\n# The session itself, run detached by a second, hidden PowerShell. The keep-\n# awake request holds for as long as it runs; once you've been idle long\n# enough it nudges the cursor 1px, which resets the idle timer chat apps read.\n# Its values come from KEEP_AWAKE_* environment variables, never its code.\n$session = {\n    Add-Type -Namespace KeepAwake -Name Native -MemberDefinition @'\n[DllImport(\"kernel32.dll\")] public static extern uint SetThreadExecutionState(uint flags);\n[DllImport(\"user32.dll\")] public static extern void mouse_event(uint flags, int dx, int dy, uint data, System.UIntPtr extra);\n[DllImport(\"user32.dll\")] public static extern bool GetLastInputInfo(ref LastInput info);\npublic struct LastInput { public uint Size; public uint Time; }\n'@\n    # ES_CONTINUOUS | ES_SYSTEM_REQUIRED, plus ES_DISPLAY_REQUIRED to keep the screen on.\n    $flags = [uint32]2147483649\n    if ($env:KEEP_AWAKE_DISPLAY -eq 'keep-on') { $flags = [uint32]2147483651 }\n    [void][KeepAwake.Native]::SetThreadExecutionState($flags)\n    $ends = [long]$env:KEEP_AWAKE_UNTIL\n    $threshold = [long]$env:KEEP_AWAKE_JIGGLE\n    $info = New-Object KeepAwake.Native+LastInput\n    $info.Size = 8\n    $wrap = [long]4294967296\n    while ($ends -eq 0 -or [DateTimeOffset]::UtcNow.ToUnixTimeSeconds() -lt $ends) {\n        Start-Sleep -Seconds 15\n        if ($threshold -gt 0 -and [KeepAwake.Native]::GetLastInputInfo([ref]$info)) {\n            $ticks = ([long][Environment]::TickCount + $wrap) % $wrap\n            $idle = (($ticks - $info.Time + $wrap) % $wrap) / 1000\n            if ($idle -ge $threshold) {\n                [KeepAwake.Native]::mouse_event(1, 1, 0, 0, [UIntPtr]::Zero)\n                Start-Sleep -Milliseconds 150\n                [KeepAwake.Native]::mouse_event(1, -1, 0, 0, [UIntPtr]::Zero)\n            }\n        }\n    }\n}\n$encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($session.ToString()))\n\n$stateDir = Join-Path $homeDir 'state'\n$state = Join-Path $stateDir 'keep-awake.json'\n$now = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds()\n\nfunction Read-State {\n    if (-not (Test-Path -LiteralPath $state)) { return $null }\n    try { return Get-Content -LiteralPath $state -Raw | ConvertFrom-Json } catch { return $null }\n}\n\n# The session's pid, if the process at the recorded pid started at the\n# recorded time and the session hasn't passed its end: a pid now used by\n# another program never matches both. (Get-Process, not WMI, which is slow.)\nfunction Get-SessionId($Saved) {\n    if (-not $Saved -or -not $Saved.pid -or -not $Saved.started) { return $null }\n    if ($Saved.until -and [long]$Saved.until -le $now) { return $null }\n    try {\n        $process = Get-Process -Id ([int]$Saved.pid)\n        if ($process.StartTime.ToUniversalTime().Ticks -eq [long]$Saved.started) { return $process.Id }\n    } catch {\n        Write-Verbose \"no session at pid $($Saved.pid): $_\"\n    }\n    return $null\n}\n\nfunction Get-Plural([long]$Count, [string]$Unit) {\n    if ($Count -eq 1) { return \"1 $Unit\" }\n    return \"$Count ${Unit}s\"\n}\n\nfunction Get-Label([string]$Seconds) {\n    if ($Seconds -eq 'inf') { return 'indefinitely' }\n    $hours = [math]::Floor([long]$Seconds / 3600)\n    $minutes = [math]::Floor(([long]$Seconds % 3600) / 60)\n    if ($hours -eq 0) { return Get-Plural $minutes 'minute' }\n    if ($minutes -eq 0) { return Get-Plural $hours 'hour' }\n    return \"$(Get-Plural $hours 'hour') $(Get-Plural $minutes 'minute')\"\n}\n\n$saved = Read-State\n$current = Get-SessionId $saved\n\nif ($action -eq 'stop') {\n    if ($current) {\n        Stop-Process -Id $current -Force -ErrorAction SilentlyContinue\n        Remove-Item -LiteralPath $state -ErrorAction SilentlyContinue\n        Write-Output 'Session ended -- this computer can sleep normally again.'\n    } else {\n        Remove-Item -LiteralPath $state -ErrorAction SilentlyContinue\n        Write-Output 'No active session -- nothing to stop.'\n    }\n    exit 0\n}\n\nif ($action -eq 'status') {\n    if (-not $current) {\n        Write-Output 'No active session.'\n        exit 0\n    }\n    if ($saved.until) {\n        # Round up, so a fresh 30-minute session reads 30m, not 29m.\n        $left = [math]::Max(0, [math]::Floor(([long]$saved.until - $now + 59) / 60))\n        $remaining = '{0}h {1:D2}m remaining of {2}' -f [math]::Floor($left / 60), [int]($left % 60), $saved.label\n    } else {\n        $remaining = 'running indefinitely'\n    }\n    Write-Output \"Session active -- $remaining.\"\n    if ($saved.jiggle) { Write-Output 'Activity simulation: on' } else { Write-Output 'Activity simulation: off' }\n    exit 0\n}\n\n# --- start ---\nNew-Item -ItemType Directory -Force -Path $stateDir | Out-Null\n$replaced = ''\nif ($current) {\n    Stop-Process -Id $current -Force -ErrorAction SilentlyContinue\n    $replaced = 'Replaced previous session -- '\n}\nRemove-Item -LiteralPath $state -ErrorAction SilentlyContinue\n\n$label = Get-Label $duration\n$until = 0\nif ($duration -ne 'inf') { $until = $now + [long]$duration }\n$env:KEEP_AWAKE_UNTIL = \"$until\"\n$env:KEEP_AWAKE_DISPLAY = $display\n$env:KEEP_AWAKE_JIGGLE = '0'\nif ($jiggle -ne 'off') { $env:KEEP_AWAKE_JIGGLE = $jiggle }\n\n# The same PowerShell that runs this step, hidden and detached: Start-Process\n# hands it no handles, so bnto sees the step finish right away.\n$powershell = (Get-Process -Id $PID).Path\n$arguments = '-NoLogo', '-NoProfile', '-NonInteractive', '-EncodedCommand', $encoded\n$process = Start-Process -FilePath $powershell -ArgumentList $arguments -WindowStyle Hidden -PassThru\nStart-Sleep -Seconds 1\nif ($process.HasExited) { Exit-Failure \"the session stopped right away (exit $($process.ExitCode))\" }\n\n$started = \"$($process.StartTime.ToUniversalTime().Ticks)\"\n$record = [ordered]@{ label = $label; pid = $process.Id; started = $started; jiggle = ($jiggle -ne 'off') }\nif ($until) { $record.until = $until }\n$record | ConvertTo-Json -Compress | Set-Content -LiteralPath $state -Encoding ASCII\n\nif ($duration -eq 'inf') { $span = 'indefinitely (until you stop it)' } else { $span = \"for $label\" }\nWrite-Output \"${replaced}Session started: staying awake $span.\"\nif ($display -eq 'keep-on') { Write-Output 'Display: kept on' } else { Write-Output 'Display: allowed to sleep' }\nif ($jiggle -eq 'off') {\n    Write-Output 'Activity simulation: off'\n} else {\n    Write-Output \"Activity simulation: on (nudges the cursor after ${jiggle}s idle)\"\n}\nWrite-Output 'Run keep-awake again with Action: Stop to end it early.'\n",
                    shell: "powershell",
                    timeout: 180,
                  },
                  position: {
                    x: 0,
                    y: 0,
                  },
                  type: "shell-run",
                  version: "1.0.0",
                },
              ],
              outputPorts: [],
              parameters: {},
              position: {
                x: 0,
                y: 300,
              },
              type: "group",
              version: "1.0.0",
              when: {
                expected: "windows",
                op: "equals",
                value: "{{ctx.platform}}",
              },
            },
          ],
          outputPorts: [],
          parameters: {},
          position: {
            x: 250,
            y: 100,
          },
          type: "group",
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
          name: "Session Log",
          outputPorts: [],
          parameters: {
            label: "Session Log",
            mode: "message",
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
