/**
 * Category-specific content for recipe pages.
 *
 * Each category has: a processing description (how-it-works step 2),
 * 3 feature highlights, and 2-3 category FAQ items.
 *
 * Adding a new category: add one block here.
 */

import type { Feature, QA } from "./recipePageContent";

interface CategoryContentBlock {
  processDescription: string;
  features: Feature[];
  faq: QA[];
}

export const CATEGORY_CONTENT: Record<string, CategoryContentBlock> = {
  image: {
    processDescription:
      "Your images are transformed instantly using a compiled Rust engine running as WebAssembly. Nothing is uploaded. Processing happens entirely in your browser.",
    features: [
      {
        heading: "Every format, one tool",
        body: "Work with JPEG, PNG, and WebP without switching between tools. bnto detects formats automatically and preserves metadata through the pipeline.",
      },
      {
        heading: "Quality you control",
        body: "Fine-tune output with adjustable quality settings. Preview results before downloading to find the perfect balance between file size and visual fidelity.",
      },
      {
        heading: "Batch processing built in",
        body: "Drop one file or a thousand. bnto processes them all in parallel using your browser's WebAssembly runtime. No per-file limits, no queues.",
      },
    ],
    faq: [
      {
        question: "Does processing reduce image quality?",
        answer:
          "It depends on the operation. Compression is lossy by design, but you control the quality slider to find the right tradeoff. Resizing and format conversion preserve visual quality at the settings you choose. You can always preview results before downloading.",
      },
      {
        question: "Can I process images larger than 10 MB?",
        answer:
          "Yes. There is no file size limit. Large images take longer to process because everything runs in your browser, but there are no artificial caps. For very large batches, the CLI is faster because it runs natively on your CPU without browser overhead.",
      },
      {
        question: "Which image formats are supported?",
        answer:
          "JPEG, PNG, WebP, GIF, AVIF, BMP, and TIFF. You can convert between any of these formats, compress them, resize them, or strip metadata. The engine handles format detection automatically.",
      },
    ],
  },

  spreadsheet: {
    processDescription:
      "Your CSV files are parsed and transformed using a compiled Rust engine running as WebAssembly. Nothing is uploaded. All processing happens in your browser tab.",
    features: [
      {
        heading: "Clean data in seconds",
        body: "Remove empty rows, trim whitespace, and deduplicate entries without writing a single formula. bnto applies consistent cleaning rules across every row.",
      },
      {
        heading: "Column operations",
        body: "Rename headers, reorder columns, and transform values in bulk. Changes apply to the entire file at once, so you never miss a row.",
      },
      {
        heading: "Format conversion",
        body: "Convert CSV to JSON and back. Merge multiple CSV files into one. Standardize column names across datasets before combining them.",
      },
    ],
    faq: [
      {
        question: "What CSV formats are supported?",
        answer:
          "Standard comma-separated values with optional header rows. The engine auto-detects delimiters and encoding. Files with semicolons, tabs, or other separators are handled automatically.",
      },
      {
        question: "Is there a row limit?",
        answer:
          "No hard limit. Processing happens in memory inside your browser, so practical limits depend on your device. Files with hundreds of thousands of rows work fine on modern hardware. For millions of rows, use the CLI for native performance.",
      },
    ],
  },

  file: {
    processDescription:
      "File operations run locally using a compiled Rust engine. In the browser, renaming happens via WebAssembly. Nothing is uploaded to any server.",
    features: [
      {
        heading: "Pattern-based renaming",
        body: "Define rename patterns once and apply them to every file in the batch. Add prefixes, suffixes, sequential numbers, or date stamps without manual editing.",
      },
      {
        heading: "Preview before applying",
        body: "See the before-and-after file names before committing. Catch mistakes early and adjust your pattern until every name looks right.",
      },
      {
        heading: "Unlimited batch size",
        body: "Rename 10 files or 10,000. The engine processes them all in a single pass. No per-file fees, no upload queues, no waiting.",
      },
    ],
    faq: [
      {
        question: "Which file types can I rename?",
        answer:
          "Any file type. Renaming changes the filename, not the contents. Images, documents, code files, archives, and any other file type work the same way.",
      },
      {
        question: "Can I undo a rename?",
        answer:
          "In the browser, renamed files are downloaded as new copies. Your originals are untouched. With the CLI, renamed files are modified in place, so use the --dry-run flag first to preview changes.",
      },
    ],
  },

  vector: {
    processDescription:
      "SVG files are parsed and rasterized using resvg and tiny-skia, compiled to WebAssembly. Nothing is uploaded. Conversion runs entirely in your browser.",
    features: [
      {
        heading: "Pixel-perfect rasterization",
        body: "Convert SVG to PNG or JPEG with precise DPI control. The engine uses the same rendering libraries as professional design tools for accurate, anti-aliased output.",
      },
      {
        heading: "SVG optimization",
        body: "Strip unnecessary metadata, comments, editor artifacts, and redundant attributes from SVG files. Reduce file size without changing how the graphic renders.",
      },
      {
        heading: "Configurable output",
        body: "Control output dimensions, DPI, background color, and format. Generate thumbnails, social media assets, or print-ready rasters from a single SVG source.",
      },
    ],
    faq: [
      {
        question: "Will my SVG look the same after conversion?",
        answer:
          "Yes. The engine uses resvg, which faithfully renders SVG features including gradients, masks, filters, and embedded fonts. Complex illustrations convert accurately.",
      },
      {
        question: "Can I set a custom DPI for the output?",
        answer:
          "Yes. The default is 96 DPI (standard screen resolution). Set higher values like 192 or 300 for retina displays or print use. Higher DPI produces larger output dimensions proportionally.",
      },
    ],
  },

  video: {
    processDescription:
      "Video operations use yt-dlp under the hood and require the CLI. Video processing is not available in the browser because it needs system-level access.",
    features: [
      {
        heading: "Download from any source",
        body: "Pass a URL and bnto handles the rest. The engine wraps yt-dlp, which supports thousands of video platforms. No manual format negotiation needed.",
      },
      {
        heading: "CLI-native performance",
        body: "Video downloads run as native processes on your system. No browser sandboxing, no WebAssembly overhead. Full disk access for writing large files directly.",
      },
      {
        heading: "Composable with other nodes",
        body: "Chain video downloads with file renaming, metadata extraction, or archival in a single recipe. Each node does one job and passes results to the next.",
      },
    ],
    faq: [
      {
        question: "Why is video download CLI-only?",
        answer:
          "Video downloading requires system-level network access and disk I/O that browsers cannot provide. The CLI runs natively on your system with full access to yt-dlp and your filesystem.",
      },
      {
        question: "Do I need to install yt-dlp separately?",
        answer:
          "Yes. Install yt-dlp via your package manager (brew install yt-dlp, pip install yt-dlp, or equivalent). The bnto doctor command checks for required dependencies.",
      },
    ],
  },
};
