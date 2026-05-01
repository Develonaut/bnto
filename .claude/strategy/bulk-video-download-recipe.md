# Bulk Video Download Recipe — Design & Gap Analysis

**Created:** April 30, 2026
**Status:** Design — blocked on data-driven iteration (Tier 0.3)
**Related:** [power-recipes.md](power-recipes.md), [engine-expansion.md](engine-expansion.md)

---

## Context

Real-world use case: downloading and organizing video collections from Patreon (or any site yt-dlp supports). A collection page has hundreds of posts organized by subject (e.g., "Suboden Khan" parts 1-3). The user wants to:

1. Scrape the collection page DOM → structured manifest (JSON or CSV)
2. Feed the manifest to bnto → create directory tree + download videos per group

This is the second validation recipe alongside the Etsy product pipeline. It exercises the same infrastructure gaps but with a simpler workflow — no HTTP API calls, no image compositing, just data-driven iteration + shell-command + filesystem ops.

---

## Current Workaround (Phase 0)

A browser console snippet + bash script. Works today, no engine changes needed.

### Step 1: DOM Scraper (browser console)

Run on a Patreon collection page after clicking "Load more" until all posts are visible:

```js
(() => {
  const title =
    document.querySelector('[elementtiming="Collection : Cover"]')?.textContent?.trim() ||
    "Untitled";
  const cards = document.querySelectorAll('a[class*="CollectionPostList"][class*="gridCard"]');
  const groups = {};

  cards.forEach((card) => {
    const postTitle = card
      .querySelector('[class*="HeadingTextBundle"][class*="sizeMd"]')
      ?.textContent?.trim();
    const url = card.href;
    if (!postTitle?.startsWith("VIDEO:")) return;

    const subject = postTitle
      .replace(/^VIDEO:\s*/, "")
      .replace(/\s*[-–]\s*[Pp]art\s*\d+\/\d+\s*$/, "")
      .replace(/\s*[Pp]art\s*\d+\/\d+\s*$/, "")
      .replace(/\s*\(?\d+\/\d+\)?\s*$/, "")
      .trim();

    if (!groups[subject]) groups[subject] = [];
    groups[subject].push(url);
  });

  const result = {
    title,
    groups: Object.entries(groups).map(([name, urls]) => ({
      title: name,
      urls,
    })),
  };

  const filename =
    title
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/-+$/, "") + ".json";
  const blob = new Blob([JSON.stringify(result, null, 2)], { type: "application/json" });
  const a = document.createElement("a");
  a.href = URL.createObjectURL(blob);
  a.download = filename;
  a.click();
  URL.revokeObjectURL(a.href);

  console.log(
    `Downloaded ${filename} — ${cards.length} posts, ${Object.keys(groups).length} groups`,
  );
})();
```

### Step 2: Download Script (bash)

```bash
./download-collection.sh space-marines.json chrome
```

Script reads the JSON with `jq`, creates subdirectories per group, and calls yt-dlp with browser cookie auth. Saved at `~/Downloads/download-collection.sh`.

### Why this works but isn't good enough

- Requires `jq` as an external dependency
- Not composable — can't chain with other bnto processors
- No progress reporting through the engine
- No retry/resilience from the engine
- Can't be shared as a reusable `.bnto.json` recipe

---

## Target Recipe (Phase 2+)

The recipe the user actually wants to author. Requires data-driven iteration and structured data ingestion.

### Input: CSV manifest

```csv
group,url
"Alpha Legion TMM Speedpaint","https://www.patreon.com/posts/154746214?collection=144005"
"Suboden Khan","https://www.patreon.com/posts/149565527?collection=144005"
"Suboden Khan","https://www.patreon.com/posts/148991800?collection=144005"
"Suboden Khan","https://www.patreon.com/posts/148430831?collection=144005"
"White Scar Mark II","https://www.patreon.com/posts/144645556?collection=144005"
```

The browser console snippet would be modified to output CSV instead of JSON. CSV is the simpler format and aligns with the existing `bnto-csv` crate ecosystem.

### Recipe Definition (target state)

```json
{
  "id": "bulk-video-download",
  "type": "group",
  "name": "Bulk Video Download",
  "metadata": {
    "description": "Download and organize videos from a CSV manifest into subdirectories",
    "category": "video",
    "tags": ["YouTube", "Patreon", "batch", "organize"]
  },
  "requires": [
    { "binary": "yt-dlp", "installHint": "brew install yt-dlp" },
    { "binary": "ffmpeg", "installHint": "brew install ffmpeg" }
  ],
  "nodes": [
    {
      "id": "input",
      "type": "input",
      "parameters": {
        "mode": "file",
        "accept": [".csv"],
        "label": "Video manifest CSV",
        "placeholder": "CSV with columns: group, url"
      }
    },
    {
      "id": "read-manifest",
      "type": "spreadsheet-read",
      "parameters": {
        "hasHeaders": true,
        "delimiter": ","
      },
      "comment": "MISSING — reads CSV bytes, emits rows as structured data"
    },
    {
      "id": "download-loop",
      "type": "loop",
      "settings": {
        "mode": "forEach",
        "items": "{{node.read-manifest.rows}}",
        "continueOnError": true
      },
      "comment": "MISSING — data-driven forEach over CSV rows",
      "nodes": [
        {
          "id": "download",
          "type": "shell-command",
          "parameters": {
            "command": "yt-dlp",
            "args": [
              "--no-playlist",
              "--no-warnings",
              "--newline",
              "--cookies-from-browser",
              "{{fields.browser}}",
              "--merge-output-format",
              "{{fields.format}}",
              "-S",
              "vcodec:{{fields.videoCodec}},acodec:{{fields.audioCodec}}",
              "-o",
              "{{fields.downloadPath}}/{{fields.title}}/{{item.group}}/%(title)s.%(ext)s",
              "{{item.url}}"
            ],
            "outputMode": "file"
          }
        }
      ]
    },
    {
      "id": "output",
      "type": "output",
      "parameters": { "mode": "download" }
    }
  ],
  "fields": {
    "title": {
      "type": "string",
      "label": "Collection Title",
      "description": "Parent directory name for the download",
      "default": "",
      "order": 1
    },
    "downloadPath": {
      "type": "string",
      "label": "Download Path",
      "description": "Base directory for downloads",
      "default": "~/Downloads",
      "order": 2
    },
    "browser": {
      "type": "enum",
      "label": "Browser Cookies",
      "description": "Browser for authenticated content",
      "options": [
        { "value": "", "label": "None" },
        { "value": "chrome", "label": "Chrome" },
        { "value": "firefox", "label": "Firefox" },
        { "value": "safari", "label": "Safari" }
      ],
      "default": "chrome",
      "order": 3
    },
    "format": {
      "type": "enum",
      "label": "Output Format",
      "options": [
        { "value": "mp4", "label": "MP4" },
        { "value": "mkv", "label": "MKV" },
        { "value": "webm", "label": "WebM" }
      ],
      "default": "mp4",
      "order": 4
    },
    "videoCodec": {
      "type": "enum",
      "label": "Video Codec",
      "options": [
        { "value": "h264", "label": "H.264" },
        { "value": "vp9", "label": "VP9" },
        { "value": "av1", "label": "AV1" }
      ],
      "default": "h264",
      "order": 5
    },
    "audioCodec": {
      "type": "enum",
      "label": "Audio Codec",
      "options": [
        { "value": "m4a", "label": "M4A (AAC)" },
        { "value": "opus", "label": "Opus" },
        { "value": "mp3", "label": "MP3" }
      ],
      "default": "m4a",
      "order": 6
    }
  },
  "edges": [
    { "source": "input", "target": "read-manifest" },
    { "source": "read-manifest", "target": "download-loop" },
    { "source": "download-loop", "target": "output" }
  ]
}
```

### Expected Usage

```bash
# Generate CSV manifest from Patreon page (browser console)
# Then run the recipe:
bnto run ./bulk-video-download.bnto.json manifest.csv \
  --param download:title="Space Marines" \
  --param download:downloadPath="/Users/Ryan/Downloads"

# Result:
# /Users/Ryan/Downloads/Space Marines/
# ├── Alpha Legion TMM Speedpaint/
# │   └── Alpha Legion TMM Speedpaint.mp4
# ├── Suboden Khan/
# │   ├── Suboden Khan - part 1.mp4
# │   ├── Suboden Khan - part 2.mp4
# │   └── Suboden Khan - part 3.mp4
# ├── White Scar Mark II/
# │   └── White Scar Mark II.mp4
# └── ...
```

---

## Gap Analysis

### What exists today

| Component                 | Status      | Notes                                                                              |
| ------------------------- | ----------- | ---------------------------------------------------------------------------------- |
| `shell-command` processor | **Exists**  | Can run yt-dlp with args, conditional groups, field templates                      |
| Field template system     | **Exists**  | `{{fields.*}}` substitution in node params                                         |
| `bnto-csv` crate          | **Exists**  | But only transforms CSV bytes (clean, rename, convert) — doesn't emit rows as data |
| Loop container            | **Exists**  | But only iterates over input files, not structured data                            |
| Node-to-node templates    | **Partial** | `{{node.<id>.*}}` syntax parsed but `node_outputs` always empty                    |

### What's missing (in priority order)

| #   | Gap                                  | Tier | Why it's needed                                                                 |
| --- | ------------------------------------ | ---- | ------------------------------------------------------------------------------- |
| 1   | **CSV row iteration**                | 0.3  | Loop over CSV rows, executing child nodes per row with `{{item.column}}` access |
| 2   | **spreadsheet-read processor**       | 1.7  | Take CSV input bytes → emit rows as structured data for the loop                |
| 3   | **Inter-node data passing**          | 0.4  | Wire `spreadsheet-read` output into `loop` container's items                    |
| 4   | **`{{item.*}}` template resolution** | 0.2  | Resolve `{{item.group}}` and `{{item.url}}` inside loop child nodes             |

These are the same gaps documented in [power-recipes.md](power-recipes.md) Tier 0.3, 0.4, 1.7. This recipe is a simpler validation than the Etsy pipeline — it only needs read + forEach + shell-command, no HTTP or filesystem nodes.

### Quickest win: CSV row iteration

The minimum engine work to make this recipe run:

1. **`spreadsheet-read` processor** — new processor in `bnto-csv` that parses CSV input bytes into `Vec<HashMap<String, String>>` and emits as `NodeOutput.data`
2. **`NodeOutput.data` field** — add `Option<serde_json::Value>` to `NodeOutput` (currently files-only)
3. **Data-driven loop** — extend `execute_loop()` to iterate over `data` from upstream node (not just input files)
4. **Loop context in templates** — populate `{{item.*}}` and `{{index}}` during loop iteration

This is exactly the Tier 0.3 + 0.4 + 1.7 infrastructure from the power-recipes gap analysis. Once it works for CSV rows, it works for any structured data source (JSON, API responses, etc.).

---

## Phased Approach

### Phase 0: Bash workaround (NOW — delivered)

- Console snippet → JSON file
- Bash script reads JSON with `jq`, creates dirs, calls yt-dlp
- Unblocks the user immediately

### Phase 1: CSV row iteration in engine (quickest engine win)

- Modify console snippet to output CSV instead of JSON
- Add `spreadsheet-read` processor (CSV bytes → structured rows)
- Add `NodeOutput.data` field
- Extend loop container for data-driven iteration
- Wire `{{item.*}}` templates in loop context
- **Result:** `bnto run ./bulk-video-download.bnto.json manifest.csv` works

### Phase 2: JSON ingestion (fuller solution)

- Add `json-parse` processor (JSON text → structured data)
- Console snippet outputs JSON directly (richer structure, nested groups)
- Nested loops for hierarchical data (groups within collections)
- **Result:** Same recipe handles both flat CSV and nested JSON manifests

### Phase 3: Recipe-as-connector (ecosystem)

- `patreon-scraper` as a recipe using headless browser or API
- `bulk-video-download` composes with `patreon-scraper`
- Community contributes scrapers for other platforms
- **Result:** `bnto run patreon-collection https://patreon.com/creator/collections/123`

---

## Relationship to Power Recipes

This recipe validates the same infrastructure as the Etsy pipeline but with a smaller surface area:

| Infrastructure          | Etsy Pipeline   | Bulk Video Download                        |
| ----------------------- | --------------- | ------------------------------------------ |
| Recipe variables        | Yes             | Yes (via fields)                           |
| Template expressions    | Yes             | Yes                                        |
| Data-driven forEach     | Yes (CSV rows)  | Yes (CSV rows)                             |
| Inter-node data passing | Yes             | Yes                                        |
| `shell-command`         | Yes             | Yes (yt-dlp)                               |
| `file-system` (mkdir)   | Yes             | No (yt-dlp creates dirs via `-o` template) |
| `spreadsheet-read`      | Yes             | Yes                                        |
| `http-request`          | Yes (Figma API) | No                                         |
| Nested loops            | Yes             | No (flat CSV)                              |

**This is the simpler proving ground.** If bulk-video-download works, the Etsy pipeline is a straightforward extension adding `http-request` and `file-system` nodes.
