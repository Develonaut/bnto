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
  readonly definition: Definition;
}

/** All engine-owned built-in recipes. */
export const GENERATED_RECIPES: readonly GeneratedRecipe[] = [
  {
    slug: "compress-images",
    name: "Compress Images",
    description: "Accepts image files and compresses each one.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "compress-image"
            },
            {
                "id": "e2",
                "source": "compress-image",
                "target": "output"
            }
        ],
        "id": "compress-images",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files and compresses each one."
        },
        "name": "Compress Images",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp"
                    ],
                    "label": "JPEG, PNG, or WebP images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "compress-image",
                "inputPorts": [],
                "metadata": {},
                "name": "Compress",
                "outputPorts": [],
                "parameters": {
                    "quality": 80
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-compress",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Compressed Images",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "resize-images",
    name: "Resize Images",
    description: "Accepts image files and resizes each one.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "resize-image"
            },
            {
                "id": "e2",
                "source": "resize-image",
                "target": "output"
            }
        ],
        "id": "resize-images",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files and resizes each one."
        },
        "name": "Resize Images",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp"
                    ],
                    "label": "JPEG, PNG, or WebP images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "resize-image",
                "inputPorts": [],
                "metadata": {},
                "name": "Resize",
                "outputPorts": [],
                "parameters": {
                    "maintainAspect": true,
                    "quality": 80,
                    "width": 200
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-resize",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Resized Images",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "convert-image-format",
    name: "Convert Image Format",
    description: "Accepts image files and converts each to a target format.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "convert-image"
            },
            {
                "id": "e2",
                "source": "convert-image",
                "target": "output"
            }
        ],
        "id": "convert-image-format",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files and converts each to a target format."
        },
        "name": "Convert Image Format",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp",
                        "image/gif"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp",
                        ".gif"
                    ],
                    "label": "JPEG, PNG, WebP, or GIF images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "convert-image",
                "inputPorts": [],
                "metadata": {},
                "name": "Convert",
                "outputPorts": [],
                "parameters": {
                    "format": "webp",
                    "quality": 80
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-convert",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Converted Images",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "rename-files",
    name: "Rename Files",
    description: "Accepts files and renames each one.",
    category: "file",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "rename-file"
            },
            {
                "id": "e2",
                "source": "rename-file",
                "target": "output"
            }
        ],
        "id": "rename-files",
        "inputPorts": [],
        "metadata": {
            "category": "file",
            "description": "Accepts files and renames each one."
        },
        "name": "Rename Files",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "*/*"
                    ],
                    "extensions": [],
                    "label": "any files",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "rename-file",
                "inputPorts": [],
                "metadata": {},
                "name": "Rename",
                "outputPorts": [],
                "parameters": {
                    "prefix": "renamed-"
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "file-rename",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Renamed Files",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "clean-csv",
    name: "Clean CSV",
    description: "Accepts a CSV file and cleans it.",
    category: "spreadsheet",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "clean"
            },
            {
                "id": "e2",
                "source": "clean",
                "target": "output"
            }
        ],
        "id": "clean-csv",
        "inputPorts": [],
        "metadata": {
            "category": "spreadsheet",
            "description": "Accepts a CSV file and cleans it."
        },
        "name": "Clean CSV",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "text/csv"
                    ],
                    "extensions": [
                        ".csv"
                    ],
                    "label": "CSV files",
                    "mode": "file-upload",
                    "multiple": false
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "clean",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Clean",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "removeDuplicates": true,
                    "removeEmptyRows": true,
                    "trimWhitespace": true
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "spreadsheet-clean",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Cleaned CSV",
                    "mode": "download",
                    "zip": false
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "rename-csv-columns",
    name: "Rename CSV Columns",
    description: "Accepts a CSV file and renames its column headers.",
    category: "spreadsheet",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "rename-columns"
            },
            {
                "id": "e2",
                "source": "rename-columns",
                "target": "output"
            }
        ],
        "id": "rename-csv-columns",
        "inputPorts": [],
        "metadata": {
            "category": "spreadsheet",
            "description": "Accepts a CSV file and renames its column headers."
        },
        "name": "Rename CSV Columns",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "text/csv"
                    ],
                    "extensions": [
                        ".csv"
                    ],
                    "label": "CSV files",
                    "mode": "file-upload",
                    "multiple": false
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "rename-columns",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Rename",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "columns": {}
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "spreadsheet-rename",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Renamed CSV",
                    "mode": "download",
                    "zip": false
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "csv-to-json",
    name: "CSV to JSON",
    description: "Accepts a CSV file and converts it to JSON.",
    category: "spreadsheet",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "convert"
            },
            {
                "id": "e2",
                "source": "convert",
                "target": "output"
            }
        ],
        "id": "csv-to-json",
        "inputPorts": [],
        "metadata": {
            "category": "spreadsheet",
            "description": "Accepts a CSV file and converts it to JSON."
        },
        "name": "CSV to JSON",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "text/csv"
                    ],
                    "extensions": [
                        ".csv"
                    ],
                    "label": "CSV files",
                    "mode": "file-upload",
                    "multiple": false
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "convert",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Convert to JSON",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "delimiter": "comma",
                    "pretty": false
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "spreadsheet-convert",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "JSON file",
                    "mode": "download",
                    "zip": false
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "merge-csv",
    name: "Merge CSV",
    description: "Accepts multiple CSV files and merges them into one.",
    category: "spreadsheet",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "merge"
            },
            {
                "id": "e2",
                "source": "merge",
                "target": "output"
            }
        ],
        "id": "merge-csv",
        "inputPorts": [],
        "metadata": {
            "category": "spreadsheet",
            "description": "Accepts multiple CSV files and merges them into one."
        },
        "name": "Merge CSV",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "text/csv"
                    ],
                    "extensions": [
                        ".csv"
                    ],
                    "label": "CSV files",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "merge",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Merge",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "deduplicate": false,
                    "headerHandling": "first-file"
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "spreadsheet-merge",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Merged CSV",
                    "mode": "download",
                    "zip": false
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "optimize-images-for-web",
    name: "Optimize Images for Web",
    description: "Accepts image files, resizes, converts to WebP, and compresses each one.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "resize"
            },
            {
                "id": "e2",
                "source": "resize",
                "target": "convert"
            },
            {
                "id": "e3",
                "source": "convert",
                "target": "compress"
            },
            {
                "id": "e4",
                "source": "compress",
                "target": "output"
            }
        ],
        "id": "optimize-images-for-web",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files, resizes, converts to WebP, and compresses each one."
        },
        "name": "Optimize Images for Web",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp"
                    ],
                    "label": "JPEG, PNG, or WebP images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "resize",
                "inputPorts": [],
                "metadata": {},
                "name": "Resize",
                "outputPorts": [],
                "parameters": {
                    "maintainAspect": true,
                    "quality": 80,
                    "width": 800
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-resize",
                "version": "1.0.0"
            },
            {
                "id": "convert",
                "inputPorts": [],
                "metadata": {},
                "name": "Convert",
                "outputPorts": [],
                "parameters": {
                    "format": "webp",
                    "quality": 80
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "image-convert",
                "version": "1.0.0"
            },
            {
                "id": "compress",
                "inputPorts": [],
                "metadata": {},
                "name": "Compress",
                "outputPorts": [],
                "parameters": {
                    "quality": 80
                },
                "position": {
                    "x": 750,
                    "y": 100
                },
                "type": "image-compress",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Optimized Images",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 1000,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "generate-thumbnails",
    name: "Generate Thumbnails",
    description: "Accepts image files, resizes to thumbnail dimensions, converts to WebP, and renames with a prefix.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "resize"
            },
            {
                "id": "e2",
                "source": "resize",
                "target": "convert"
            },
            {
                "id": "e3",
                "source": "convert",
                "target": "rename"
            },
            {
                "id": "e4",
                "source": "rename",
                "target": "output"
            }
        ],
        "id": "generate-thumbnails",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files, resizes to thumbnail dimensions, converts to WebP, and renames with a prefix."
        },
        "name": "Generate Thumbnails",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp"
                    ],
                    "label": "JPEG, PNG, or WebP images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "resize",
                "inputPorts": [],
                "metadata": {},
                "name": "Resize",
                "outputPorts": [],
                "parameters": {
                    "maintainAspect": true,
                    "quality": 80,
                    "width": 150
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-resize",
                "version": "1.0.0"
            },
            {
                "id": "convert",
                "inputPorts": [],
                "metadata": {},
                "name": "Convert",
                "outputPorts": [],
                "parameters": {
                    "format": "webp",
                    "quality": 80
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "image-convert",
                "version": "1.0.0"
            },
            {
                "id": "rename",
                "inputPorts": [],
                "metadata": {},
                "name": "Rename",
                "outputPorts": [],
                "parameters": {
                    "prefix": "thumb_"
                },
                "position": {
                    "x": 750,
                    "y": 100
                },
                "type": "file-rename",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Thumbnails",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 1000,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "compress-and-rename",
    name: "Compress & Rename",
    description: "Accepts image files, compresses each one, and renames with a suffix.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "compress"
            },
            {
                "id": "e2",
                "source": "compress",
                "target": "rename"
            },
            {
                "id": "e3",
                "source": "rename",
                "target": "output"
            }
        ],
        "id": "compress-and-rename",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files, compresses each one, and renames with a suffix."
        },
        "name": "Compress & Rename",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp"
                    ],
                    "label": "JPEG, PNG, or WebP images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "compress",
                "inputPorts": [],
                "metadata": {},
                "name": "Compress",
                "outputPorts": [],
                "parameters": {
                    "quality": 80
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-compress",
                "version": "1.0.0"
            },
            {
                "id": "rename",
                "inputPorts": [],
                "metadata": {},
                "name": "Rename",
                "outputPorts": [],
                "parameters": {
                    "suffix": "-min"
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "file-rename",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Compressed & Renamed",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 750,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "standardize-csv",
    name: "Standardize CSV",
    description: "Accepts a CSV file, cleans it, and renames the column headers.",
    category: "spreadsheet",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "clean"
            },
            {
                "id": "e2",
                "source": "clean",
                "target": "rename-columns"
            },
            {
                "id": "e3",
                "source": "rename-columns",
                "target": "output"
            }
        ],
        "id": "standardize-csv",
        "inputPorts": [],
        "metadata": {
            "category": "spreadsheet",
            "description": "Accepts a CSV file, cleans it, and renames the column headers."
        },
        "name": "Standardize CSV",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "text/csv"
                    ],
                    "extensions": [
                        ".csv"
                    ],
                    "label": "CSV files",
                    "mode": "file-upload",
                    "multiple": false
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "clean",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Clean",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "removeDuplicates": true,
                    "removeEmptyRows": true,
                    "trimWhitespace": true
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "spreadsheet-clean",
                "version": "1.0.0"
            },
            {
                "id": "rename-columns",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Rename",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "columns": {}
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "spreadsheet-rename",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Standardized CSV",
                    "mode": "download",
                    "zip": false
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "strip-exif",
    name: "Strip EXIF",
    description: "Accepts image files and strips EXIF metadata from each one.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "strip-exif-node"
            },
            {
                "id": "e2",
                "source": "strip-exif-node",
                "target": "output"
            }
        ],
        "id": "strip-exif",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files and strips EXIF metadata from each one."
        },
        "name": "Strip EXIF",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp"
                    ],
                    "label": "JPEG, PNG, or WebP images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "strip-exif-node",
                "inputPorts": [],
                "metadata": {},
                "name": "Strip EXIF",
                "outputPorts": [],
                "parameters": {
                    "quality": 80
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-strip-exif",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Cleaned Images",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "watermark-images",
    name: "Watermark Images",
    description: "Accepts image files and overlays an image onto each one.",
    category: "image",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "overlay"
            },
            {
                "id": "e2",
                "source": "overlay",
                "target": "output"
            }
        ],
        "id": "watermark-images",
        "inputPorts": [],
        "metadata": {
            "category": "image",
            "description": "Accepts image files and overlays an image onto each one."
        },
        "name": "Watermark Images",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [
                        "image/jpeg",
                        "image/png",
                        "image/webp"
                    ],
                    "extensions": [
                        ".jpg",
                        ".jpeg",
                        ".png",
                        ".webp"
                    ],
                    "label": "JPEG, PNG, or WebP images",
                    "mode": "file-upload",
                    "multiple": true
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "overlay",
                "inputPorts": [],
                "metadata": {},
                "name": "Overlay",
                "outputPorts": [],
                "parameters": {
                    "offsetX": 0,
                    "offsetY": 0,
                    "opacity": 80,
                    "overlay": "",
                    "position": "bottom-right",
                    "quality": 80,
                    "size": 25
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "image-overlay",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Watermarked Images",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
  {
    slug: "download-video",
    name: "Download Video",
    description: "Downloads video or audio from a URL via yt-dlp.",
    category: "video",
    definition: {
        "edges": [
            {
                "id": "e1",
                "source": "input",
                "target": "video-download"
            },
            {
                "id": "e2",
                "source": "video-download",
                "target": "output"
            }
        ],
        "id": "download-video",
        "inputPorts": [],
        "metadata": {
            "category": "video",
            "description": "Downloads video or audio from a URL via yt-dlp."
        },
        "name": "Download Video",
        "nodes": [
            {
                "id": "input",
                "inputPorts": [],
                "metadata": {},
                "name": "Input",
                "outputPorts": [
                    {
                        "id": "out-1",
                        "name": "files"
                    }
                ],
                "parameters": {
                    "accept": [],
                    "extensions": [],
                    "label": "Video URL",
                    "mode": "url",
                    "multiple": true,
                    "placeholder": "https://youtube.com/watch?v=..."
                },
                "position": {
                    "x": 0,
                    "y": 100
                },
                "type": "input",
                "version": "1.0.0"
            },
            {
                "id": "video-download",
                "inputPorts": [],
                "metadata": {},
                "name": "Download Video",
                "outputPorts": [],
                "parameters": {
                    "format": "mp4",
                    "quality": "best",
                    "url": ""
                },
                "position": {
                    "x": 250,
                    "y": 100
                },
                "type": "video-download",
                "version": "1.0.0"
            },
            {
                "id": "output",
                "inputPorts": [
                    {
                        "id": "in-1",
                        "name": "files"
                    }
                ],
                "metadata": {},
                "name": "Output",
                "outputPorts": [],
                "parameters": {
                    "autoDownload": true,
                    "label": "Downloaded Video",
                    "mode": "download",
                    "zip": true
                },
                "position": {
                    "x": 500,
                    "y": 100
                },
                "type": "output",
                "version": "1.0.0"
            }
        ],
        "outputPorts": [],
        "parameters": {},
        "position": {
            "x": 0,
            "y": 0
        },
        "settings": {
            "iteration": "auto"
        },
        "type": "group",
        "version": "1.0.0"
    },
  },
] as const;
